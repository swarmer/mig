pub mod codes;

use std;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Decoding(String),
    UnsupportedVersion(u32),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref io_error) => io_error.fmt(f),
            Error::Decoding(ref message) => message.fmt(f),
            Error::UnsupportedVersion(version) => {
                write!(f, "Unsupported version: {}", version)
            },
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref io_error) => io_error.description(),
            Error::Decoding(ref message) => message,
            Error::UnsupportedVersion(..) => "Unsupported version",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref io_error) => Some(io_error),
            Error::Decoding(..) => None,
            Error::UnsupportedVersion(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
