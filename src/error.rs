use std::error::Error;
use std::fmt;
use std::io;

use c_ares;

#[derive(Debug)]
pub enum ResolverError {
    Io(io::Error),
    Ares(c_ares::AresError),
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolverError::Io(ref err) => err.fmt(f),
            ResolverError::Ares(ref err) => err.fmt(f),
        }
    }
}

impl Error for ResolverError {
    fn description(&self) -> &str {
        match *self {
            ResolverError::Io(ref err) => err.description(),
            ResolverError::Ares(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ResolverError::Io(ref err) => Some(err),
            ResolverError::Ares(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for ResolverError {
    fn from(err: io::Error) -> ResolverError {
        ResolverError::Io(err)
    }
}

impl From<c_ares::AresError> for ResolverError {
    fn from(err: c_ares::AresError) -> ResolverError {
        ResolverError::Ares(err)
    }
}
