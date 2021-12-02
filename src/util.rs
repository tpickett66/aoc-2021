use std::io;
use std::result;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error {
    details: String
}

impl Error {
    #[allow(unused)]
    fn new(msg: &str) -> Error {
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
        Error{ details: io_err.to_string() }
    }
}

pub type Result<T> = result::Result<T, Error>;
