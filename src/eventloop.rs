use std::collections::HashMap;
use std::io::ErrorKind;
#[cfg(unix)]
use std::os::fd::BorrowedFd;
#[cfg(windows)]
use std::os::windows::io::BorrowedSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(cares1_34)]
use c_ares::{FdEventFlags, FdEvents, ProcessFlags};

use crate::error::Error;
use polling::Event;

// Indicate an interest in read and/or write events.
struct Interest(bool, bool);

// Object returned when the EventLoop is run.  When this is dropped, the EventLoop is stopped.
pub struct EventLoopStopper {
    poller: Arc<polling::Poller>,
    quit: Arc<AtomicBool>,
}

impl EventLoopStopper {
    pub fn new(poller: Arc<polling::Poller>, quit: Arc<AtomicBool>) -> Self {
        Self { poller, quit }
    }
}

impl Drop for EventLoopStopper {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Relaxed);
        self.poller.notify().expect("Failed to notify poller");
    }
}

// The EventLoop sets up a polling::Poller and use it to wait for events on sockets as directed by
// the c-ares library.
pub struct EventLoop {
    poller: Arc<polling::Poller>,
    interests: Arc<Mutex<HashMap<c_ares::Socket, Interest>>>,
    pub ares_channel: Arc<Mutex<c_ares::Channel>>,
    quit: Arc<AtomicBool>,

    #[allow(dead_code)]
    pending_write: Arc<AtomicBool>,
}

impl EventLoop {
    // Create a new event loop.
    pub fn new(mut options: c_ares::Options) -> Result<Self, Error> {
        // Create a polling::Poller on which to wait for events, and a hashmap to record which
        // sockets we are interested in.
        let poller = Arc::new(polling::Poller::new()?);
        let interests: HashMap<c_ares::Socket, Interest> = HashMap::new();
        let interests = Arc::new(Mutex::new(interests));

        // Whenever c-ares tells us that it cares about a socket, we'll update the poller
        // accordingly.
        //
        // Safety: we are trusting c-ares to give us a socket that is valid and that will remain
        // open until we are asked to drop our interest.
        {
            let poller = Arc::clone(&poller);
            let interests = Arc::clone(&interests);
            let sock_callback = move |socket: c_ares::Socket, readable: bool, writable: bool| {
                let mut interests = interests.lock().unwrap();
                if !readable && !writable {
                    if interests.remove(&socket).is_some() {
                        let source = unsafe { borrow_socket(socket) };
                        poller
                            .delete(source)
                            .expect("Failed to remove socket from poller");
                    }
                } else {
                    let key = usize::try_from(socket).unwrap();
                    let event = Event::new(key, readable, writable);
                    let interest = Interest(readable, writable);
                    if interests.insert(socket, interest).is_none() {
                        unsafe {
                            poller
                                .add(socket, event)
                                .expect("failed to add socket to poller");
                        }
                    } else {
                        let source = unsafe { borrow_socket(socket) };
                        poller
                            .modify(source, event)
                            .expect("failed to update interest");
                    }
                }
            };
            options.set_socket_state_callback(sock_callback);
        }

        // Create the c-ares channel.
        #[allow(unused_mut)]
        let mut ares_channel = c_ares::Channel::with_options(options)?;

        // Implement the pending-write optimization.
        let pending_write = Arc::new(AtomicBool::new(false));
        #[cfg(cares1_34)]
        {
            let pending_write = Arc::clone(&pending_write);
            let poller = Arc::clone(&poller);
            let pending_write_callback = move || {
                pending_write.store(true, Ordering::Relaxed);
                poller
                    .notify()
                    .expect("Failed to notify poller of pending write");
            };
            ares_channel.set_pending_write_callback(pending_write_callback);
        }

        // Create and return the event loop.
        let locked_channel = Arc::new(Mutex::new(ares_channel));
        let event_loop = Self {
            poller,
            interests,
            ares_channel: locked_channel,
            quit: Arc::new(AtomicBool::new(false)),
            pending_write,
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) -> EventLoopStopper {
        // Create a stopper.
        let poller = Arc::clone(&self.poller);
        let quit = Arc::clone(&self.quit);
        let stopper = EventLoopStopper::new(poller, quit);

        thread::spawn(|| self.event_loop_thread());
        stopper
    }

    // Event loop thread - waits for events, and handles them.
    fn event_loop_thread(mut self) {
        let mut events = polling::Events::new();
        let timeout = Duration::from_millis(500);
        loop {
            // Wait for something to happen.
            events.clear();
            let results = self.poller.wait(&mut events, Some(timeout));

            // If we're asked to quit, then quit.
            if self.quit.load(Ordering::Relaxed) {
                break;
            }

            // Interrupted is OK, we just retry.  Other errors are unexpected.
            if let Err(ref err) = results {
                if err.kind() == ErrorKind::Interrupted {
                    continue;
                }
            }
            results.expect("Poll failed");

            // Process any pending write.
            #[cfg(cares1_34)]
            if self.pending_write.swap(false, Ordering::Relaxed) {
                self.ares_channel.lock().unwrap().process_pending_write();
            }

            // Process any events.
            self.handle_events(&events);

            // `polling` always operates in oneshot mode, but c-ares expects us to maintain an
            // interest in sockets until told otherwise.
            //
            // So re-assert our interest in all reported sockets.
            {
                let interests = self.interests.lock().unwrap();
                for event in events.iter() {
                    let socket = c_ares::Socket::try_from(event.key).unwrap();
                    if let Some(Interest(readable, writable)) = interests.get(&socket) {
                        // Safety: we trust that since c-ares hasn't yet told us that it is done
                        // with this socket, it's still open.
                        let source = unsafe { borrow_socket(socket) };
                        let new_event = Event::new(event.key, *readable, *writable);
                        self.poller
                            .modify(source, new_event)
                            .expect("failed to renew interest");
                    }
                }
            }
        }
    }

    #[cfg(cares1_34)]
    fn handle_events(&mut self, events: &polling::Events) {
        let mut fd_events: Vec<FdEvents> = Vec::with_capacity(events.capacity().into());
        let fd_events_iter = events.iter().map(|event| {
            let socket = c_ares::Socket::try_from(event.key).unwrap();
            let mut event_flags = FdEventFlags::empty();
            if event.readable {
                event_flags.insert(FdEventFlags::Read)
            }
            if event.writable {
                event_flags.insert(FdEventFlags::Write)
            }
            FdEvents::new(socket, event_flags)
        });
        fd_events.extend(fd_events_iter);

        let _ = self
            .ares_channel
            .lock()
            .unwrap()
            .process_fds(&fd_events, ProcessFlags::empty());
    }

    #[cfg(not(cares1_34))]
    fn handle_events(&mut self, events: &polling::Events) {
        let mut acted = false;
        for event in events.iter() {
            let socket = c_ares::Socket::try_from(event.key).unwrap();

            let rfd = if event.readable {
                socket
            } else {
                c_ares::SOCKET_BAD
            };

            let wfd = if event.writable {
                socket
            } else {
                c_ares::SOCKET_BAD
            };

            self.ares_channel.lock().unwrap().process_fd(rfd, wfd);
            acted = true;
        }

        if !acted {
            // No events.  Have c-ares process any timeouts.
            self.ares_channel
                .lock()
                .unwrap()
                .process_fd(c_ares::SOCKET_BAD, c_ares::SOCKET_BAD);
        }
    }
}

#[cfg(unix)]
unsafe fn borrow_socket(socket: c_ares::Socket) -> impl polling::AsSource {
    unsafe { BorrowedFd::borrow_raw(socket) }
}

#[cfg(windows)]
unsafe fn borrow_socket(socket: c_ares::Socket) -> impl polling::AsSource {
    unsafe { BorrowedSocket::borrow_raw(socket) }
}
