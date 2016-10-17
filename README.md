# c-ares-resolver #

A more convenient API around [`c-ares`](https://dimbleby/rust-c-ares/), for asynchronous DNS requests.

[![Build Status](https://travis-ci.org/dimbleby/c-ares-resolver.svg?branch=master)](https://travis-ci.org/dimbleby/c-ares-resolver)
[![Build status](https://ci.appveyor.com/api/projects/status/m9o3f4u6wuofq8k9/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/c-ares-resolver/branch/master)
[![crates.io](http://meritbadge.herokuapp.com/c-ares-resolver)](https://crates.io/crates/c-ares-resolver)

## Documentation ##

- API documentation is [here](http://dimbleby.github.io/c-ares-resolver).
- There are some example programs [here](https://github.com/dimbleby/c-ares-resolver/tree/master/examples).

## Installation ##

To use `c-ares-resolver`, add this to your `Cargo.toml`:

```toml
[dependencies]
c-ares-resolver = "*"
```

And add this to your crate root:

```rust
extern crate c_ares_resolver;
```

## Contributing ##

Contributions are welcome.  Please send pull requests!
