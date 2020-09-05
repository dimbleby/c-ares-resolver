use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::error::Error;

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

// The EventLoop sets up a polling::Poller and use it to wait for events on file descriptors as
// directed by the c-ares library.
pub struct EventLoop {
    poller: Arc<polling::Poller>,
    interests: Arc<Mutex<HashMap<c_ares::Socket, Interest>>>,
    pub ares_channel: Arc<Mutex<c_ares::Channel>>,
    quit: Arc<AtomicBool>,
}

impl EventLoop {
    // Create a new event loop.
    pub fn new(mut options: c_ares::Options) -> Result<Self, Error> {
        // Create a polling::Poller on which to wait for events, and a hashmap to record what
        // sockets are interested in.
        let poller = Arc::new(polling::Poller::new()?);
        let interests: HashMap<c_ares::Socket, Interest> = HashMap::new();
        let interests = Arc::new(Mutex::new(interests));

        // Whenever c-ares tells us what to do with a file descriptor, we'll update the poller
        // accordingly.
        {
            let poller = Arc::clone(&poller);
            let interests = Arc::clone(&interests);
            let sock_callback = move |socket: c_ares::Socket, readable: bool, writable: bool| {
                let mut interests = interests.lock().unwrap();
                if !readable && !writable {
                    if interests.remove(&socket).is_some() {
                        poller
                            .remove(socket)
                            .expect("Failed to remove socket from poller");
                    }
                } else {
                    let interest = Interest(readable, writable);
                    if interests.insert(socket, interest).is_none() {
                        poller
                            .insert(socket)
                            .expect("failed to add socket to poller");
                    }
                    let event = polling::Event {
                        key: socket as usize,
                        readable,
                        writable,
                    };
                    poller
                        .interest(socket, event)
                        .expect("failed to register interest");
                }
            };
            options.set_socket_state_callback(sock_callback);
        }

        // Create the c-ares channel.
        let ares_channel = c_ares::Channel::with_options(options)?;
        let locked_channel = Arc::new(Mutex::new(ares_channel));

        // Create and return the event loop.
        let event_loop = EventLoop {
            poller,
            interests,
            ares_channel: locked_channel,
            quit: Arc::new(AtomicBool::new(false)),
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
        let mut events = vec![];
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
            let results = results.expect("Poll failed");

            // Process any events.
            match results {
                0 => {
                    // No events.  Have c-ares process any timeouts.
                    self.ares_channel
                        .lock()
                        .unwrap()
                        .process_fd(c_ares::SOCKET_BAD, c_ares::SOCKET_BAD);
                }
                _ => {
                    // Process events.
                    for event in &events {
                        self.handle_event(&event);
                    }
                }
            }
        }
    }

    // Handle a single event.
    fn handle_event(&mut self, event: &polling::Event) {
        // `polling` always operates in oneshot mode, but c-ares expects us to maintain an interest
        // in sockets until told otherwise.
        //
        // So re-assert our interest in this socket.
        let socket = event.key as c_ares::Socket;
        {
            let interests = self.interests.lock().unwrap();
            if let Some(Interest(readable, writable)) = interests.get(&socket) {
                let new_event = polling::Event {
                    key: event.key,
                    readable: *readable,
                    writable: *writable,
                };
                self.poller
                    .interest(socket, new_event)
                    .expect("failed to register interest");
            }
        }

        // Tell c-ares that something happened.
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
    }
}
