use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::error::Error;

// Indicate an interest in read and/or write events.
struct Interest(bool, bool);

// The EventLoop sets up a polling::Poller and use it to wait for events on file descriptors as
// directed by the c-ares library.
pub struct EventLoop {
    poller: Arc<polling::Poller>,
    rx_msg_channel: crossbeam_channel::Receiver<polling::Event>,
    interests: HashMap<c_ares::Socket, Interest>,
    pub ares_channel: Arc<Mutex<c_ares::Channel>>,
    quit: Arc<AtomicBool>,
}

// Object returned when the EventLoop is run.  Call `stop()` to stop the associated EventLoop.
pub struct EventLoopStopper {
    poller: Arc<polling::Poller>,
    quit: Arc<AtomicBool>,
}

impl EventLoopStopper {
    pub fn new(poller: Arc<polling::Poller>, quit: Arc<AtomicBool>) -> Self {
        Self { poller, quit }
    }

    pub fn stop(&self) {
        self.quit.store(true, Ordering::Relaxed);
        let _ = self.poller.notify();
    }
}

impl EventLoop {
    // Create a new event loop.
    pub fn new(mut options: c_ares::Options) -> Result<Self, Error> {
        // Create a polling::Poller on which to wait for events, and a channel for sending messages
        // to the event loop.
        let poller = Arc::new(polling::Poller::new()?);
        let (tx, rx) = crossbeam_channel::unbounded();

        // Whenever c-ares tells us what to do with a file descriptor, we'll send a message
        // registering our interest, and wake up the poller.
        {
            let poller = Arc::clone(&poller);
            let sock_callback = move |socket: c_ares::Socket, readable: bool, writable: bool| {
                let event = polling::Event {
                    key: socket as usize,
                    readable,
                    writable,
                };
                let _ = tx.send(event);
                let _ = poller.notify();
            };
            options.set_socket_state_callback(sock_callback);
        }

        // Create the c-ares channel.
        let ares_channel = c_ares::Channel::with_options(options)?;
        let locked_channel = Arc::new(Mutex::new(ares_channel));

        // Create and return the event loop.
        let event_loop = EventLoop {
            poller,
            rx_msg_channel: rx,
            interests: HashMap::new(),
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
            let results = self
                .poller
                .wait(&mut events, Some(timeout))
                .expect("poll failed");

            // If we're asked to quit, then quit.
            if self.quit.load(Ordering::Relaxed) {
                break;
            }

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

            // Process any messages.
            self.handle_messages();
        }
    }

    // Handle a single event.
    fn handle_event(&mut self, event: &polling::Event) {
        // `polling` always operates in oneshot mode, but c-ares expects us to maintain an interest
        // in sockets until told otherwise.
        //
        // So re-assert our interest in this socket.
        let socket = event.key as c_ares::Socket;
        if let Some(Interest(readable, writable)) = self.interests.get(&socket) {
            let new_event = polling::Event {
                key: event.key,
                readable: *readable,
                writable: *writable,
            };
            self.poller
                .interest(socket, new_event)
                .expect("failed to register interest");
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

    // Process messages incoming on the channel.
    fn handle_messages(&mut self) {
        for event in self.rx_msg_channel.try_iter() {
            let socket = event.key as c_ares::Socket;
            if !event.readable && !event.writable {
                self.interests.remove(&socket);
                let _ = self.poller.remove(socket);
            } else {
                let interest = Interest(event.readable, event.writable);
                if self.interests.insert(socket, interest).is_none() {
                    self.poller
                        .insert(socket)
                        .expect("failed to add socket to poller");
                }
                self.poller
                    .interest(socket, event)
                    .expect("failed to register interest");
            }
        }
    }
}
