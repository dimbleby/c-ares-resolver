use std::sync::{
    Arc,
    Mutex,
};

use c_ares;
use futures;
use futures::Future;

use error::ResolverError;
use eventloop::{
    EventLoop,
    EventLoopHandle
};

/// Used to configure the behaviour of the resolver.
#[derive(Default)]
pub struct Options {
    inner: c_ares::Options,
}

impl Options {
    /// Returns a fresh `Options`, on which no values are set.
    pub fn new() -> Self {
        Self::default()
    }
}

/// An asynchronous DNS resolver.
pub struct Resolver {
    ares_channel: Arc<Mutex<c_ares::Channel>>,
    event_loop_handle: EventLoopHandle,
}

impl Resolver {
    /// Create a new Resolver.
    pub fn new(options: Options) -> Result<Resolver, ResolverError> {
        // Create and run the event loop.
        let event_loop = try!(EventLoop::new(options.inner));
        let channel = event_loop.ares_channel.clone();
        let handle = event_loop.run();

        // Return the Resolver.
        let resolver = Resolver {
            ares_channel: channel,
            event_loop_handle: handle,
        };
        Ok(resolver)
    }

    pub fn query_cname(&self, name: &str)
        -> futures::BoxFuture<c_ares::CNameResults, c_ares::AresError> {
        let (c, p) = futures::oneshot();
        self.ares_channel.lock().unwrap().query_cname(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::AresError::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    pub fn query_mx(&self, name: &str)
        -> futures::BoxFuture<c_ares::MXResults, c_ares::AresError> {
        let (c, p) = futures::oneshot();
        self.ares_channel.lock().unwrap().query_mx(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::AresError::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }

    pub fn query_naptr(&self, name: &str)
        -> futures::BoxFuture<c_ares::NAPTRResults, c_ares::AresError> {
        let (c, p) = futures::oneshot();
        self.ares_channel.lock().unwrap().query_naptr(name, move |result| {
            c.complete(result);
        });
        p.map_err(|_| c_ares::AresError::ECANCELLED)
            .and_then(futures::done)
            .boxed()
    }
}

impl Drop for Resolver {
    fn drop(&mut self) {
        // Shut down the event loop.
        self.event_loop_handle.shutdown();
    }
}
