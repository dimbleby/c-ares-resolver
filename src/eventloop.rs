#[cfg(unix)]
pub use crate::unix::eventloop::EventLoop;

#[cfg(windows)]
pub use crate::windows::eventloop::EventLoop;
