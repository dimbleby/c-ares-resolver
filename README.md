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

[![Crates.io][crates-badge]][crates-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/c-ares-resolver.svg
[crates-url]: https://crates.io/crates/c-ares-resolver
[actions-badge]: https://github.com/dimbleby/c-ares-resolver/actions/workflows/build.yml/badge.svg
[actions-url]: https://github.com/dimbleby/c-ares-resolver/actions?query=workflow%3ACI+branch%3Amaster

## Documentation

API documentation is [here](https://docs.rs/c-ares-resolver).

Setting the feature `build-cmake` will cause the `c-ares` library to be built
using `cmake`.
This is significantly faster than the default `autotools` build on unix
platforms: so if it works for you, you should probably prefer it.

## Examples

```rust
extern crate c_ares_resolver;
extern crate futures_executor;
use futures_executor::block_on;

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
