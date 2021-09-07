//! DNS resolvers built on [`c-ares`](https://c-ares.org), for asynchronous DNS requests.
//!
//! This crate provides three resolver types - the `Resolver`, the `FutureResolver`, and the
//! `BlockingResolver`:
//!
//! * The `Resolver` is the thinnest wrapper around the underlying `c-ares` library.  It returns
//! answers via callbacks.  The other resolvers are built on top of this.
//!
//! * The `FutureResolver` returns answers as `std::future::Future`s.
//!
//! * The `BlockingResolver` isn't asynchronous at all - as the name suggests, it blocks until the
//! lookup completes.
//!
//! On all resolvers:
//!
//! -  methods like `query_xxx` correspond to the `c-ares` function `ares_query`, which "initiates
//! a single-question DNS query".
//!
//! -  methods like `search_xxx` correspond to the `c-ares` function `ares_search`, which
//! "initiates a series of single-question DNS queries ... using the channel's search domains as
//! well as a host alias file given by the HOSTALIAS environment variable".
//!
//! See [`c-ares` documentation](https://c-ares.org/docs.html) for more details.
//!
//! # Example
//!
//! ```rust
//! extern crate c_ares_resolver;
//! extern crate futures_executor;
//! use futures_executor::block_on;
//!
//! fn main() {
//!     let resolver = c_ares_resolver::FutureResolver::new().unwrap();
//!     let query = resolver.query_a("google.com");
//!     let response = block_on(query);
//!     match response {
//!         Ok(result) => println!("{}", result),
//!         Err(e) => println!("Lookup failed with error '{}'", e)
//!     }
//! }
//! ```
//!
//! Further examples showing how to use the library can be found
//! [here](https://github.com/dimbleby/c-ares-resolver/tree/master/examples).
#![deny(missing_docs)]
extern crate c_ares;
extern crate crossbeam_channel;
extern crate futures_channel;
extern crate polling;

mod blockingresolver;
mod error;
mod eventloop;
mod futureresolver;
mod host;
mod nameinfo;
mod resolver;

#[cfg(test)]
mod tests;

pub use crate::blockingresolver::BlockingResolver;
pub use crate::error::Error;
pub use crate::futureresolver::{CAresFuture, FutureResolver};
pub use crate::host::HostResults;
pub use crate::nameinfo::NameInfoResult;
pub use crate::resolver::{Options, Resolver};
