use std::error;
use std::fmt;
use std::io;

/// Error codes that the library might return.
#[derive(Debug)]
pub enum Error {
    /// An `io::Error`.
    Io(io::Error),

    /// A `c_ares::Error`.
    Ares(c_ares::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Io(ref err) => err.fmt(f),
            Self::Ares(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Io(ref err) => Some(err),
            Self::Ares(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<c_ares::Error> for Error {
    fn from(err: c_ares::Error) -> Self {
        Self::Ares(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn error_is_send() {
        assert_send::<Error>();
    }

    #[test]
    fn error_is_sync() {
        assert_sync::<Error>();
    }

    #[test]
    fn display_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = Error::Io(io_err);
        let display = format!("{}", err);
        assert!(display.contains("file not found"));
    }

    #[test]
    fn display_ares_error() {
        let ares_err = c_ares::Error::ENODATA;
        let err = Error::Ares(ares_err);
        let display = format!("{}", err);
        assert!(!display.is_empty());
    }

    #[test]
    fn source_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test error");
        let err = Error::Io(io_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn source_ares_error() {
        let ares_err = c_ares::Error::ENODATA;
        let err = Error::Ares(ares_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "test");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn from_ares_error() {
        let ares_err = c_ares::Error::ENOTFOUND;
        let err: Error = ares_err.into();
        assert!(matches!(err, Error::Ares(_)));
    }

    #[test]
    fn debug_format() {
        let io_err = io::Error::new(io::ErrorKind::Other, "test");
        let err = Error::Io(io_err);
        let debug = format!("{:?}", err);
        assert!(debug.contains("Io"));
    }
}
