//! A convenient wrapper for the [`c-ares`](http://c-ares.haxx.se) library.
//!
//! The [`c-ares` crate](https://crates.io/crates/c-ares) provides a safe
//! wrapper around the underlying C library, but it's relatively hard work to
//! use: the user needs to drive the polling of file descriptors according to
//! `c-ares` demands, which likely involves writing something close to a
//! full-blown event loop.
//!
//! This crate does that hard work for you so that the presented API is much
//! more straightforward.  Simply create a `Resolver`, and make your query.
//!
//! In most cases, the returned value is a `futures::Future`.
//!
//! In some exceptional cases, the value returned by the `c-ares` library lives
//! on the stack, so that its lifetime does not allow it to be returned via a
//! `Future`.  In such cases this crate's API takes a callback.
//!
//! Complete examples showing how to use the library can be found
//! [here](https://github.com/dimbleby/tokio-c-ares/tree/master/examples).
extern crate c_ares;
extern crate futures;
extern crate mio;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate ws2_32;

mod error;
mod eventloop;
mod futureresolver;
mod resolver;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub use error::Error;
pub use futureresolver::FutureResolver;
pub use resolver::{
    Options,
    Resolver,
};
