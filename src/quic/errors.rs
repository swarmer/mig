use std;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Decoding(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref io_error) => io_error.fmt(f),
            Error::Decoding(ref message) => message.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref io_error) => io_error.description(),
            Error::Decoding(ref message) => message,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref io_error) => io_error.cause(),
            Error::Decoding(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
