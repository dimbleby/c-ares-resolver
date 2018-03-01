use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[cfg(unix)]
pub use unix::eventloop::EventLoop;

#[cfg(windows)]
pub use windows::eventloop::EventLoop;

pub struct EventLoopHandle {
    handle: Option<thread::JoinHandle<()>>,
    quit: Arc<AtomicBool>,
}

impl EventLoopHandle {
    pub fn new(handle: thread::JoinHandle<()>, quit: Arc<AtomicBool>) -> EventLoopHandle {
        EventLoopHandle {
            handle: Some(handle),
            quit,
        }
    }
}

impl Drop for EventLoopHandle {
    fn drop(&mut self) {
        if let Some(_handle) = self.handle.take() {
            self.quit.store(true, Ordering::Relaxed);
        }
    }
}
