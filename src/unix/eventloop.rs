use std::collections::HashSet;
use std::sync::{
    Arc,
    Mutex,
};
use std::thread;
use std::time::Duration;

use c_ares;
use mio;
use mio_more;

use error::Error;
use eventloop::EventLoopHandle;

// The EventLoop will set up a mio::Poll and use it to wait for the following:
//
// -  messages telling it which file descriptors it should be interested in.
//    These file descriptors are then registered (or deregistered) with the
//    mio::Poll as required.
//
// -  events telling it that something has happened on one of these file
//    descriptors.  When this happens, it tells the c_ares::Channel about it.
//
// -  a message telling it to shut down.
pub struct EventLoop {
    poll: mio::Poll,
    tx_msg_channel: mio_more::channel::Sender<Message>,
    rx_msg_channel: mio_more::channel::Receiver<Message>,
    tracked_fds: HashSet<c_ares::Socket>,
    pub ares_channel: Arc<Mutex<c_ares::Channel>>,
    quit: bool,
}

// Messages for the event loop.
#[derive(Debug)]
pub enum Message {
    // 'Notify me when this file descriptor becomes readable, or writable'.
    // The first bool is for 'readable' and the second is for 'writable'.  It's
    // allowed to set both of these - or neither, meaning 'I am no longer
    // interested in this file descriptor'.
    RegisterInterest(c_ares::Socket, bool, bool),

    // 'Shut down'.
    ShutDown,
}

// A token identifying that the message channel has become available for
// reading.
//
// We use Token(fd) for file descriptors, so this relies on zero not being a
// valid file descriptor for c-ares to use.  Zero is stdin, so that's true.
const CHANNEL: mio::Token = mio::Token(0);

impl EventLoop {
    // Create a new event loop.
    pub fn new(mut options: c_ares::Options) -> Result<EventLoop, Error> {
        // Create a mio::Poll on which to wait for events, and register a
        // channel with it.
        let poll = mio::Poll::new()?;
        let (tx, rx) = mio_more::channel::channel();
        poll.register(
            &rx,
            CHANNEL,
            mio::Ready::readable(),
            mio::PollOpt::edge()
        )?;

        // Whenever c-ares tells us what to do with a file descriptor, we'll
        // send that request along, through the channel we just created.
        let tx_clone = tx.clone();
        let sock_callback =
            move |fd: c_ares::Socket, readable: bool, writable: bool| {
                let _ = tx_clone.send(
                    Message::RegisterInterest(fd, readable, writable));
            };
        options.set_socket_state_callback(sock_callback);

        // Create the c-ares channel.
        let ares_channel = c_ares::Channel::with_options(options)?;
        let locked_channel = Arc::new(Mutex::new(ares_channel));

        // Create and return the event loop.
        let event_loop = EventLoop {
            poll: poll,
            tx_msg_channel: tx,
            rx_msg_channel: rx,
            tracked_fds: HashSet::<c_ares::Socket>::new(),
            ares_channel: locked_channel,
            quit: false,
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) -> EventLoopHandle {
        let tx_clone = self.tx_msg_channel.clone();
        let join_handle = thread::spawn(|| self.event_loop_thread());
        EventLoopHandle::new(join_handle, tx_clone)
    }

    // Event loop thread - waits for events, and handles them.
    fn event_loop_thread(mut self) {
        let mut events = mio::Events::with_capacity(16);
        loop {
            // Wait for something to happen.
            let timeout = Duration::from_millis(100);
            let results = self.poll
                .poll(&mut events, Some(timeout))
                .expect("poll failed");

            // Process whatever happened.
            match results {
                0 => {
                    // No events - must be a timeout.  Tell c-ares about it.
                    self.ares_channel.lock().unwrap().process_fd(
                        c_ares::SOCKET_BAD,
                        c_ares::SOCKET_BAD
                    );
                },
                _ => {
                    // Process events.  One of them might have asked us to
                    // quit.
                    for event in &events {
                        self.handle_event(&event);
                        if self.quit { return }
                    }
                }
            }
        }
    }

    // Handle a single event.
    fn handle_event(&mut self, event: &mio::Event) {
        match event.token() {
            CHANNEL => {
                // The channel is readable.
                self.handle_messages()
            },

            mio::Token(fd) => {
                // Sockets became readable or writable - tell c-ares.
                let ready = mio::unix::UnixReady::from(event.readiness());
                let error = ready.is_error() || ready.is_hup();
                let rfd = if error || ready.is_readable() {
                    fd as c_ares::Socket
                } else {
                    c_ares::SOCKET_BAD
                };
                let wfd = if error || ready.is_writable() {
                    fd as c_ares::Socket
                } else {
                    c_ares::SOCKET_BAD
                };
                self.ares_channel.lock().unwrap().process_fd(rfd, wfd);
            }
        }
    }

    // Process messages incoming on the channel.
    fn handle_messages(&mut self) {
        loop {
            match self.rx_msg_channel.try_recv() {
                // Instruction to do something with a file descriptor.
                Ok(Message::RegisterInterest(fd, readable, writable)) => {
                    let efd = mio::unix::EventedFd(&fd);
                    if !readable && !writable {
                        self.tracked_fds.remove(&fd);
                        let _ = self.poll.deregister(&efd);
                    } else {
                        assert_ne!(fd, 0);
                        let token = mio::Token(fd as usize);
                        let mut interest = mio::Ready::empty();
                        if readable { interest.insert(mio::Ready::readable()) }
                        if writable { interest.insert(mio::Ready::writable()) }
                        interest.insert(
                            mio::unix::UnixReady::error() | mio::unix::UnixReady::hup()
                        );
                        let register_result = if !self.tracked_fds.insert(fd) {
                            self.poll.reregister(
                                &efd,
                                token,
                                interest,
                                mio::PollOpt::level()
                            )
                        } else {
                            self.poll.register(
                                &efd,
                                token,
                                interest,
                                mio::PollOpt::level()
                            )
                        };
                        register_result.expect("failed to register interest");
                    }
                },

                // Instruction to shut down.
                Ok(Message::ShutDown) => {
                    self.quit = true;
                    break
                },

                // No more instructions.
                Err(_) => break,
            }
        }
    }
}
