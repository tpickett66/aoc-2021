use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::ops::Range;
use std::result;

use crate::util::{Error, Result};

const ROW_RANGES: [Range<usize>; 5] = [(0..5), (5..10), (10..15), (15..20), (20..25)];
const COLUMN_BASE_RANGE: [usize; 5] = [0, 5, 10, 15, 20];

pub fn run<T: Read>(reader: BufReader<T>, verbosity: u64) -> Result<Board> {
    let mut lines = reader.lines();
    let picked_numbers_string = lines.next().unwrap().unwrap();
    if verbosity >= 1 { println!("Selected numbers: {}", picked_numbers_string) }

    let picked_numbers = match picked_numbers_string
        .split(",")
        .map(|v| v.parse::<u8>())
        .collect::<result::Result<Vec<_>, _>>() {
        Ok(r) => { r }
        Err(e) => { return Err(Error::from(e)) }
    };

    // Discard the blank line between the inputs and the first board.
    let _discard = lines.next();

    let mut boards = Vec::new();
    {
        let mut board_lines = Vec::<String>::new();

        for line_res in lines {
            let line = line_res.unwrap();

            if line.is_empty() {
                let board = Board::try_from(&board_lines)?;
                boards.push(board);
                board_lines.clear();
            } else {
                board_lines.push(line);
            }
        }
    }

    if verbosity >= 1 { println!("Number of boards found: {}", boards.len()); }

    let mut winning_boards: Vec<Board> = Vec::new();

    for num in picked_numbers {
        for board in boards.iter_mut() {
            board.play(num);
            if board.is_bingo() {
                winning_boards.push(board.clone());
            }
        }

        if winning_boards.len() > 0 {
            break;
        }
    }

    if winning_boards.is_empty() {
        return Err(Error::new("No winning boards found!"))
    }

    winning_boards.sort_by_key(|b| b.score());
    let winning_board = winning_boards[0].clone();
    Ok(winning_board)
}

#[derive(Debug,Clone)]
pub struct Board {
    cells: Vec<u8>,
    claimed_cells: Vec<bool>,
    last_called: u8
}

impl Board {
    fn claimed_cell_indices(&self) -> Vec<usize> {
        self.claimed_cells
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Play a number against the board, returning whether or not the number is present on the board
    fn play(&mut self, val: u8) {
        return match self.cells.iter().position(|&v| v == val) {
            None => { }
            Some(idx) => {
                self.claimed_cells[idx] = true;
                self.last_called = val;
            }
        }
    }

    fn is_bingo(&self) -> bool {
        let claimed_indices = self.claimed_cell_indices();

        for rng in ROW_RANGES {
            let mut r = rng.clone();
            if r.all(|v| claimed_indices.contains(&v)) {
                return true
            }
        }

        for incr in 0..5 {
            if COLUMN_BASE_RANGE.iter().all(|&v| {
                let column_idx = v + incr;
                claimed_indices.contains(&column_idx)
            }) {
                return true
            }
        }

        false
    }

    fn score(&self) -> Option<u64> {
        if !self.is_bingo() {
            return None
        }
        let claimed_indices = self.claimed_cell_indices();
        let unplayed_numbers = self.cells
            .iter()
            .enumerate()
            .filter(|(idx, _)| !claimed_indices.contains(idx))
            .map(|(_, &v)| v)
            .fold(0u64, |a, v| a+v as u64);
        Some(unplayed_numbers * self.last_called as u64)
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.cells.iter().zip(other.cells.iter())
            .all(|(&t, &o)| t == o)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let score = match self.score() {
            Some(s) => { s.to_string() }
            None => { String::from("none") }
        };
        write!(f, "Score: {}", score)
    }
}

impl TryFrom<&Vec<String>> for Board {
    type Error = crate::util::Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        let mut cells = Vec::<u8>::new();
        for line in value {
            let mut row = match line
                .split_whitespace()
                .map(|v| v.parse::<u8>())
                .collect::<result::Result<Vec<_>, _>>() {
                Ok(r) => { r }
                Err(e) => { return Err(Error::from(e)) }
            };
            cells.append(&mut row);
        }

        let b = Board {
            cells,
            claimed_cells: vec![false; 25],
            last_called: 0
        };
        Ok(b)
    }
}

#[cfg(test)]
mod test {
    use crate::day4::Board;

    const WINNING_BOARD: [u8; 25] = [
        14, 21, 17, 24, 4,
        10, 16, 15, 9, 19,
        18, 8, 23, 26, 20,
        22, 11, 13, 6, 5,
        2, 0, 12, 3, 7
    ];

    fn build_test_board() -> Board {
        let cells = vec![
            22, 13, 17, 11, 0,
            8, 2, 23, 4, 24,
            21, 9, 14, 16, 7,
            6, 10, 3, 18,  5,
            1, 12, 20, 15, 19
        ];

        Board {
            cells,
            claimed_cells: vec![false; 25],
            last_called: 0
        }
    }

    #[test]
    fn test_board_try_from_vec_of_strings() {
        let expected = build_test_board();

        let lines: Vec<String> = vec!["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", " 1 12 20 15 19"]
            .iter()
            .map(|&v| String::from(v))
            .collect();
        match Board::try_from(&lines) {
            Ok(b) => { assert_eq!(b, expected) }
            Err(e) => { assert!(false, "Building the board failed: {}", e) }
        }
    }

    #[test]
    fn test_board_play() {
        let mut board = build_test_board();

        board.play(22);
        assert!(board.claimed_cells[0]);
        let claimed_count = board.claimed_cells
            .iter()
            .filter(|&&v| v)
            .collect::<Vec<_>>()
            .len();
        assert_eq!(claimed_count, 1);
        board.play(32);
        let claimed_count2 = board.claimed_cells
            .iter()
            .filter(|&&v| v)
            .collect::<Vec<_>>()
            .len();
        assert_eq!(claimed_count2, 1);
    }

    #[test]
    fn test_board_is_bingo() {
        let mut board1 = build_test_board();
        let mut board2 = build_test_board();

        // Row bingo
        for v in vec![22, 13, 17, 11, 0] {
            board1.play(v);
        }
        assert!(board1.is_bingo(), "Expected full row being played to be bingo but wasn't.");

        // Column bingo
        for v in vec![22, 8, 21, 6, 1] {
            board2.play(v);
        }
        assert!(board2.is_bingo(), "Expected full column being played to be bingo but wasn't.");
    }

    #[test]
    fn test_board_score() {
        let plays = vec![7,4,9,5,11,17,23,2,0,14,21,24];
        let mut board = Board {
            cells: WINNING_BOARD.to_vec(),
            claimed_cells: vec![false; 25],
            last_called: 0
        };

        for play in plays {
            board.play(play);
        }

        assert!(board.is_bingo(), "Expected the board to be a bingo after the play list but wasn't.");
        match board.score() {
            None => { assert!(false, "Expected a bingo board to have a score but None was returned!") }
            Some(s) => { assert_eq!(s, 4512) }
        }
    }
}
