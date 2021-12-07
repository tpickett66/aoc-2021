use crate::util::Result;
use std::io::{BufReader, Read};

pub fn run<T: Read>(reader: BufReader<T>, _verbosity: u64) -> Result<()> {
    Ok(())
}
