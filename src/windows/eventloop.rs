use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use winapi::um::winsock2::{
    fd_set, select, timeval, WSACleanup, WSAStartup, SOCKET_ERROR, WSADATA,
};

use crate::error::Error;

// The EventLoop will use select() to check on the status of file descriptors that c-ares cares
// about.
pub struct EventLoop {
    pub ares_channel: Arc<Mutex<c_ares::Channel>>,
    quit: Arc<AtomicBool>,
}

impl EventLoop {
    // Create a new event loop.
    pub fn new(options: c_ares::Options) -> Result<Self, Error> {
        // Initialize sockets.
        unsafe {
            let mut wsadata = mem::MaybeUninit::<WSADATA>::uninit();
            WSAStartup(0x101, wsadata.as_mut_ptr());
        }

        // Create the c-ares channel.
        let ares_channel = c_ares::Channel::with_options(options)?;
        let locked_channel = Arc::new(Mutex::new(ares_channel));

        // Create and return the event loop.
        let event_loop = EventLoop {
            ares_channel: locked_channel,
            quit: Arc::new(AtomicBool::new(false)),
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) -> Arc<AtomicBool> {
        let stopper = Arc::clone(&self.quit);
        thread::spawn(|| self.event_loop_thread());
        stopper
    }

    // Event loop thread - waits for events, and handles them.
    fn event_loop_thread(self) {
        let mut read_fds: fd_set = unsafe { mem::MaybeUninit::zeroed().assume_init() };
        let mut write_fds: fd_set = unsafe { mem::MaybeUninit::zeroed().assume_init() };

        // Loop round, asking c-ares what it cares about and doing as asked.
        loop {
            read_fds.fd_count = 0;
            write_fds.fd_count = 0;
            let count = self
                .ares_channel
                .lock()
                .unwrap()
                .fds(&mut read_fds, &mut write_fds);

            if count == 0 {
                thread::sleep(Duration::from_millis(100));
            } else {
                let timeout = timeval {
                    tv_sec: 0,
                    tv_usec: 100_000,
                };
                let results =
                    unsafe { select(0, &mut read_fds, &mut write_fds, ptr::null_mut(), &timeout) };

                // Process whatever happened.
                match results {
                    SOCKET_ERROR => panic!("Socket error"),
                    _ => self
                        .ares_channel
                        .lock()
                        .unwrap()
                        .process(&mut read_fds, &mut write_fds),
                }
            }
            if self.quit.load(Ordering::Relaxed) {
                break;
            }
        }
    }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        unsafe {
            WSACleanup();
        }
    }
}
