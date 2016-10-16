# tokio-c-ares #

A convenient futures-based API around [`c-ares`](https://dimbleby/rust-c-ares/), for asynchronous DNS requests.

[![Build Status](https://travis-ci.org/dimbleby/tokio-c-ares.svg?branch=master)](https://travis-ci.org/dimbleby/tokio-c-ares)
[![Build status](https://ci.appveyor.com/api/projects/status/me4646je4dhpeks7/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/tokio-c-ares/branch/master)
[![crates.io](http://meritbadge.herokuapp.com/tokio-c-ares)](https://crates.io/crates/tokio-c-ares)

## Documentation ##

- API documentation is [here](http://dimbleby.github.io/tokio-c-ares).
- There are some example programs [here](https://github.com/dimbleby/tokio-c-ares/tree/master/examples).

## Installation ##

To use `tokio-c-ares`, add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-c-ares = "*"
```

And add this to your crate root:

```rust
extern crate tokio_c_ares;
```

## Contributing ##

Contributions are welcome.  Please send pull requests!
