use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};
use std::result;

use crate::util::{Error, Result};

pub fn run<T: Read>(_reader: BufReader<T>, _verbosity: u64) -> Result<Board> {
    let s = String::from("18  3 22  4 34");
    let split = s.split_whitespace();

    for v in split {
        println!("'{}'", v);
    }

    let e = Error::new("oops");
    Err(e)
}

#[derive(Debug)]
pub struct Board {
    rows: Vec<Vec<u8>>
}

impl Board {
    // fn new() -> Self
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.rows.iter().zip(other.rows.iter())
            .all(|(t, o)| t == o)
    }
}

impl Display for Board {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TryFrom<Vec<String>> for Board {
    type Error = crate::util::Error;

    fn try_from(value: Vec<String>) -> std::prelude::rust_2015::Result<Self, Self::Error> {
        let mut rows = Vec::<Vec<u8>>::new();
        for line in value {
            let row = match line
                .split_whitespace()
                .map(|v| v.parse::<u8>())
                .collect::<result::Result<Vec<_>, _>>() {
                Ok(r) => { r }
                Err(e) => { return Err(Error::from(e)) }
            };
            rows.push(row);
        }
        let b = Board {
            rows
        };
        Ok(b)
    }
}

#[cfg(test)]
mod test {
    use crate::day4::Board;

    #[test]
    fn test_board_try_from_vec_of_strings() {
        let expected = Board {
            rows: vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18,  5],
                vec![1, 12, 20, 15, 19]
            ]
        };
        let lines: Vec<String> = vec!["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", " 1 12 20 15 19"]
            .iter()
            .map(|&v| String::from(v))
            .collect();
        match Board::try_from(lines) {
            Ok(b) => { assert_eq!(b, expected) }
            Err(e) => { assert!(false, "Building the board failed: {}", e) }

        }
    }
}
