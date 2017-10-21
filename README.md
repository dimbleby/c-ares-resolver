# c-ares-resolver #

A more convenient API around [`c-ares`](https://github.com/dimbleby/rust-c-ares/), for asynchronous DNS requests.

The `c-ares` crate provides a safe wrapper around the underlying C library, but it's relatively hard work to use: the user needs to drive the polling of file descriptors according to `c-ares` demands, which likely involves writing something close to a full-blown event loop.

This crate does that hard work for you so that the presented API is much more straightforward. 

[![Build Status](https://travis-ci.org/dimbleby/c-ares-resolver.svg?branch=master)](https://travis-ci.org/dimbleby/c-ares-resolver)
[![Build status](https://ci.appveyor.com/api/projects/status/m9o3f4u6wuofq8k9/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/c-ares-resolver/branch/master)
[![crates.io](http://meritbadge.herokuapp.com/c-ares-resolver)](https://crates.io/crates/c-ares-resolver)

## Documentation ##

API documentation is [here](http://dimbleby.github.io/c-ares-resolver).

## Examples ##

```rust
extern crate c_ares_resolver;
extern crate tokio_core;

fn main() {
    let resolver = c_ares_resolver::FutureResolver::new().unwrap();
    let query = resolver.query_a("google.com");
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let result = event_loop.run(query).unwrap();
    println!("{}", result);
}
```

Further example programs can be found [here](https://github.com/dimbleby/c-ares-resolver/tree/master/examples).

## Contributing ##

Contributions are welcome.  Please send pull requests!
