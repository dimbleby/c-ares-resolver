# Changelog

## 8.5.0 (26 May 2024)

- add `get_servers()`

## 8.4.0 (24 May 2024)

- cares 1.29.0

## 8.3.0 (23 February 2024)

- cares 1.27.0

## 8.2.0 (30 November 2023)

- c-ares 1.23.0

## 8.1.0 (14 November 2023)

- c-ares 1.22.0

## 8.0.0 (11 November 2023)

- Support versions of c-ares back to 1.13.0
  - Breaking if you are using features from a new c-ares but building in an
    environment where an old c-ares is available
  - Then this crate will by default attempt to use the old c-ares: you should
    either remove the old c-ares from your environment, or set the `vendored`
    feature flag.

## 7.6.0 (14 Oct 2023)

- Drop crossbeam-channel in favour of standard library channels
  - As of 1.67.0, the standard library's implementation is based on
    crossbeam-channel
- Take upstream c-ares 1.19.0
  - Introduces `Options::set_udp_max_queries()`

## 7.5.0 (29 Jan 2023)

- Take upstream c-ares 1.19.0
  - In particular, introduces `Options::set_hosts_path()`

## 7.4.1 (6 Nov 2021)

- feature "build-cmake" to use the cmake-based build for c-ares

## 7.4.0 (26 Oct 2021)

- Update dependencies
- Expose `set_sortlist()`

## 7.3.0 (23 Aug 2021)

- `cargo diet` to reduce crate size
- Add support for URI records

## 7.2.1 (20 Jan 2021)

- Only pull in the `futures` crates that we use

## 7.2.0 (29 Nov 2020)

- Update dependencies
- Add support for CAA records

## 7.1.3 (5 Sep 2020)

- Bug fix
  - Handle being interrupted while mid-poll()

## 7.1.2 (16 Aug 2020)

- Bug fix
  - We had a window where our poller could hold a file descriptor that c-ares
    had already closed

## 7.1.1 (16 Aug 2020)

- Wake up the event loop less often

## 7.1.0 (15 Aug 2020)

- Modernize error handling: `description()` and `cause()` are deprecated, we now
  use `Display` and `source()`.
- Fix docs to say that we now use `std::future::Future`s.

## 7.0.0 (17 Nov 2019)

- Move to using `std::future::Future`s.

## 6.1.0 (2 Nov 2018)

- Take upstream c-ares 1.15.0
  - In particular, introduces `Options::set_resolvconf_path()`

## 6.0.0 (1 July 2018)

- Take small arguments by value, per clippy's `trivially_copy_pass_by_ref`
- Bump c-ares dependency

## 5.0.0 (27 May 2018)

- Bump dependencies (bump to `c-ares` is a breaking change)

## 4.0.3 (11 May 2018)

- Improved docs and examples
- Minor reworking of event loop shutdown

## 4.0.2 (7 Apr 2018)

- Bump dependencies (fixes minimal-versions build on OSX)

## 4.0.1 (7 Apr 2018)

- Bump dependencies (fixes minimal-versions build)

## 4.0.0 (4 Jan 2018)

- winapi 0.3.3
- start maintaining a CHANGELOG
