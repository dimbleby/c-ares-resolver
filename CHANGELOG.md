# Changelog

## Unreleased

- Update `polling`, `crossbeam-channel`

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
