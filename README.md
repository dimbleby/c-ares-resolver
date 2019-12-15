# c-ares-resolver

DNS resolvers built on [`c-ares`](https://github.com/dimbleby/rust-c-ares/), for
asynchronous DNS requests.

This crate provides three resolver types - the `Resolver`, the `FutureResolver`,
and the `BlockingResolver`:

- The `Resolver` is the thinnest wrapper around the underlying `c-ares` library.
  It returns answers via callbacks. The other resolvers are built on top of
  this.
- The `FutureResolver` returns answers as `std::future::Future`s.
- The `BlockingResolver` isn't asynchronous at all - as the name suggests, it
  blocks until the lookup completes.

[![Build Status](https://travis-ci.org/dimbleby/c-ares-resolver.svg?branch=master)](https://travis-ci.org/dimbleby/c-ares-resolver)
[![Build status](https://ci.appveyor.com/api/projects/status/m9o3f4u6wuofq8k9/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/c-ares-resolver/branch/master)
[![crates.io](https://meritbadge.herokuapp.com/c-ares-resolver)](https://crates.io/crates/c-ares-resolver)

## Documentation

API documentation is [here](https://docs.rs/c-ares-resolver).

## Examples

```rust
extern crate c_ares_resolver;
extern crate futures;
use futures::executor::block_on;

fn main() {
    let resolver = c_ares_resolver::FutureResolver::new().unwrap();
    let query = resolver.query_a("google.com");
    let response = block_on(query);
    match response {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Lookup failed with error '{}'", e)
    }
}
```

Further example programs can be found
[here](https://github.com/dimbleby/c-ares-resolver/tree/master/examples).

## Contributing

Contributions are welcome. Please send pull requests!
