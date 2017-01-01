use std;
use std::fmt;
use std::rc::Rc;

use docopt;


#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub exit_code: i32,
    pub cause: Rc<docopt::Error>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&std::error::Error> {
        Some(&*self.cause)
    }
}

impl From<docopt::Error> for Error {
    fn from(error: docopt::Error) -> Error {
        Error { message: "TODO".to_string(), exit_code: 1, cause: Rc::new(error) }
    }
}


pub type Result<T> = std::result::Result<T, Error>;
