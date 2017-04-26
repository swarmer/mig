pub mod codes;

use std;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum Error {
    BufferOverflow,
    Decoding(String),
    InvalidData(String),
    InvalidHandle,
    InvalidStream,
    Io(io::Error),
    UnsupportedVersion(u32),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::convert::Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
            Error::BufferOverflow => io::Error::new(
                io::ErrorKind::InvalidData,
                self,
            ),
            Error::Decoding(..) => io::Error::new(
                io::ErrorKind::InvalidData,
                self,
            ),
            Error::InvalidData(..) => io::Error::new(
                io::ErrorKind::InvalidData,
                self,
            ),
            Error::InvalidHandle => io::Error::new(
                io::ErrorKind::InvalidInput,
                self,
            ),
            Error::InvalidStream => io::Error::new(
                io::ErrorKind::InvalidInput,
                self,
            ),
            Error::Io(io_error) => io_error,
            Error::UnsupportedVersion(..) => io::Error::new(
                io::ErrorKind::InvalidData,
                self,
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferOverflow => write!(f, "Incoming buffer overflow"),
            Error::Decoding(ref message) => message.fmt(f),
            Error::InvalidData(ref message) => message.fmt(f),
            Error::InvalidHandle => write!(f, "Invalid handle"),
            Error::InvalidStream => write!(f, "Invalid stream"),
            Error::Io(ref io_error) => io_error.fmt(f),
            Error::UnsupportedVersion(version) => write!(f, "Unsupported version: {}", version),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BufferOverflow => "Incoming buffer overflow",
            Error::Decoding(ref message) => message,
            Error::InvalidData(ref message) => message,
            Error::InvalidHandle => "Invalid handle",
            Error::InvalidStream => "Invalid stream",
            Error::Io(ref io_error) => io_error.description(),
            Error::UnsupportedVersion(..) => "Unsupported version",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::BufferOverflow => None,
            Error::Decoding(..) => None,
            Error::InvalidData(..) => None,
            Error::InvalidHandle => None,
            Error::InvalidStream => None,
            Error::Io(ref io_error) => Some(io_error),
            Error::UnsupportedVersion(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
