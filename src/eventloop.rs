use std::thread;

use mio;

#[cfg(unix)]
use unix::eventloop::Message;

#[cfg(unix)]
pub use unix::eventloop::EventLoop;

#[cfg(windows)]
use windows::eventloop::Message;

#[cfg(windows)]
pub use windows::eventloop::EventLoop;

pub struct EventLoopHandle {
    handle: thread::JoinHandle<()>,
    tx_msg_channel: mio::channel::Sender<Message>,
}

impl EventLoopHandle {
    pub fn new(
        handle: thread::JoinHandle<()>,
        tx_msg_channel: mio::channel::Sender<Message>) -> EventLoopHandle {
        EventLoopHandle {
            handle: handle,
            tx_msg_channel: tx_msg_channel,
        }
    }

    pub fn shutdown(self) {
        self.tx_msg_channel
            .send(Message::ShutDown)
            .expect("failed to request event loop to shut down");
        self.handle.join().expect("failed to shut down event loop");
    }
}
