use std::mem;
use std::ptr;
use std::sync::{
    Arc,
    Mutex,
};
use std::thread;
use std::time::Duration;

use winapi::winsock2::{
    fd_set,
    SOCKET_ERROR,
    timeval,
    WSADATA,
};
use ws2_32::{
    select,
    WSACleanup,
    WSAStartup,
};

use c_ares;
use mio;

use error::ResolverError;
use eventloop::EventLoopHandle;

// The EventLoop will use select() to check on the status of file descriptors
// that c-ares cares about.
//
// It also waits for a message telling it to shut down.  We use a mio channel
// here only for consistency with the unix interface.
pub struct EventLoop {
    tx_msg_channel: mio::channel::Sender<Message>,
    rx_msg_channel: mio::channel::Receiver<Message>,
    pub ares_channel: Arc<Mutex<c_ares::Channel>>,
    quit: bool,
}

// Messages for the event loop.
#[derive(Debug)]
pub enum Message {
    // 'Shut down'.
    ShutDown,
}

impl EventLoop {
    // Create a new event loop.
    pub fn new(options: c_ares::Options) -> Result<EventLoop, ResolverError> {
        // Initialize sockets.
        unsafe {
            let mut wsadata: WSADATA = mem::uninitialized();
            WSAStartup(0x101, &mut wsadata);
        }

        // Create the message channel.
        let (tx, rx) = mio::channel::channel();

        // Create the c-ares channel.
        let ares_channel = try!(c_ares::Channel::new(options));
        let locked_channel = Arc::new(Mutex::new(ares_channel));

        // Create and return the event loop.
        let event_loop = EventLoop {
            tx_msg_channel: tx,
            rx_msg_channel: rx,
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
        let mut read_fds: fd_set = unsafe { mem::uninitialized() };
        let mut write_fds: fd_set = unsafe { mem::uninitialized() };

        // Loop round, asking c-ares what it cares about and doing as asked.
        loop {
            read_fds.fd_count = 0;
            write_fds.fd_count = 0;
            let count = self.ares_channel.lock().unwrap()
                .fds(&mut read_fds, &mut write_fds);

            if count == 0 {
                thread::sleep(Duration::from_millis(100));
            } else {
                let select_timeout = timeval {
                    tv_sec: 0,
                    tv_usec: 100000,
                };
                let results = unsafe {
                    select(
                        0,
                        &mut read_fds,
                        &mut write_fds,
                        ptr::null_mut(),
                        &select_timeout
                    )
                };

                // Process whatever happened.
                match results {
                    SOCKET_ERROR => panic!("Socket error"),
                    _ => self.ares_channel.lock().unwrap()
                        .process(&mut read_fds, &mut write_fds),
                }
            }

            // Check whether we've been asked to quit.
            self.handle_messages();
            if self.quit { break }
        }
    }

    // Process messages incoming on the channel.
    fn handle_messages(&mut self) {
        // The only possible message is an instruction to shut down.
        if let Ok(Message::ShutDown) = self.rx_msg_channel.try_recv() {
            self.quit = true;
        }
    }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        unsafe { WSACleanup(); }
    }
}
