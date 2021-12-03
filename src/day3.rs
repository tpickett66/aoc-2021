use crate::util;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};


pub fn run(input_path: &str, _verbosity: u64) -> util::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);


    Ok(())
}

fn calculate_gamma<T: Read>(reader: BufReader<T>, _verbosity: u64) -> util::Result<u64> {
    Ok(0)
}
