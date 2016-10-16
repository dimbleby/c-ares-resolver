extern crate c_ares;
extern crate futures;
extern crate mio;
extern crate tokio_core;

mod error;
mod eventloop;
mod resolver;

#[cfg(unix)]
mod unix;

pub use error::ResolverError;
pub use resolver::{
    Options,
    Resolver,
};
