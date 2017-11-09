use std::thread;

use mio_more;

#[cfg(unix)]
use unix::eventloop::Message;

#[cfg(unix)]
pub use unix::eventloop::EventLoop;

#[cfg(windows)]
use windows::eventloop::Message;

#[cfg(windows)]
pub use windows::eventloop::EventLoop;

pub struct EventLoopHandle {
    handle: Option<thread::JoinHandle<()>>,
    tx_msg_channel: mio_more::channel::Sender<Message>,
}

impl EventLoopHandle {
    pub fn new(
        handle: thread::JoinHandle<()>,
        tx_msg_channel: mio_more::channel::Sender<Message>) -> EventLoopHandle {
        EventLoopHandle {
            handle: Some(handle),
            tx_msg_channel: tx_msg_channel,
        }
    }
}

impl Drop for EventLoopHandle {
    fn drop(&mut self) {
        if let Some(_handle) = self.handle.take() {
            let _ = self.tx_msg_channel.send(Message::ShutDown);
        }
    }
}
