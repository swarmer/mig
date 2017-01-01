use std;
use std::fmt;

use docopt;


#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub exit_code: i32,
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
        None
    }
}

impl From<docopt::Error> for Error {
    fn from(error: docopt::Error) -> Error {
        match error {
            docopt::Error::WithProgramUsage(err_box, message) => {
                let exit_code = match *err_box {
                    docopt::Error::Usage(..) => panic!("Invalid usage string!"),
                    docopt::Error::Argv(..) => 1,
                    docopt::Error::NoMatch => 1,
                    docopt::Error::Decode(..) => 1,
                    docopt::Error::WithProgramUsage(..) => unreachable!(),
                    docopt::Error::Help => 0,
                    docopt::Error::Version(..) => 0,
                };
                Error { message: message, exit_code: exit_code }
            },
            _ => {
                panic!("Unknown docopt error!");
            }
        }
    }
}


pub type Result<T> = std::result::Result<T, Error>;
