extern crate c_ares;
extern crate futures;
extern crate mio;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate ws2_32;

mod error;
mod eventloop;
mod resolver;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub use error::Error;
pub use resolver::{
    Options,
    Resolver,
};
