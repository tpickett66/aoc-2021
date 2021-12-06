use std::io;
use std::result;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Error {
    details: String
}

impl Error {
    pub fn new<T: ToString>(msg: T) -> Error {
        Error{details: msg.to_string()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Self::new(io_err)
    }
}

impl From<std::num::ParseIntError> for Error {
    // mmmm pie
    fn from(pie: ParseIntError) -> Self {
        Self::new(pie)
    }
}

pub type Result<T> = result::Result<T, Error>;
