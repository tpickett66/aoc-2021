use std::fmt::{Display, Formatter};
use crate::util::{Error, Result};
use std::io::{BufReader, Read};

pub fn run<T: Read>(reader: BufReader<T>, verbosity: u64) -> Result<Board> {
    let e = Error::new("oops");
    Err(e)
}

pub struct Board {
    rows: Vec<Vec<u8>>
}

impl Board {
    // fn new() -> Self
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<Vec<String>> for Board {
    fn from(lines: Vec<String>) -> Self {
        todo!()
    }
}
