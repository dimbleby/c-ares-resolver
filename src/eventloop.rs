use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[cfg(unix)]
pub use unix::eventloop::EventLoop;

#[cfg(windows)]
pub use windows::eventloop::EventLoop;

// Dropping the EventLoopHandle causes the event loop to quit.
pub struct EventLoopHandle {
    _handle: thread::JoinHandle<()>,
    quit: Arc<AtomicBool>,
}

impl EventLoopHandle {
    pub fn new(handle: thread::JoinHandle<()>, quit: Arc<AtomicBool>) -> EventLoopHandle {
        EventLoopHandle {
            _handle: handle,
            quit,
        }
    }
}

impl Drop for EventLoopHandle {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Relaxed);
    }
}
