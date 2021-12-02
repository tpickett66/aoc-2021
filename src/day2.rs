use std::fmt::{Display, Formatter};
use crate::util::Result;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn run(input_path: &str, verbosity: u64) -> Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let position = calculate_position(reader, verbosity)?;

    let combined_position = position.depth * position.horizontal;
    println!("Vertical Position:   {}", position.depth);
    println!("Horizontal Position: {}", position.horizontal);
    println!("Combined Position:   {}", combined_position);

    Ok(())
}

fn calculate_position<T: Read>(reader: BufReader<T>, verbosity: u64) -> Result<Position> {
    let mut pos = Position::default();

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let amount = parts.next().unwrap().parse::<i64>().unwrap();

        match direction {
            "forward" => {
                pos.horizontal += amount;
                pos.depth += amount * pos.aim;
            }
            "up" => { pos.aim -= amount }
            "down" => { pos.aim += amount }
            _ => {}
        }

        if verbosity >= 1 {
            println!("Current position: {} Direction: {} Distance: {}", pos, direction, amount)
        }
    }
    Ok(pos)
}

struct Position {
    aim: i64,
    depth: i64,
    horizontal: i64,
}

impl Default for Position {
    fn default() -> Self {
        Position{aim: 0, depth: 0, horizontal: 0}
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{depth: {}, horizontal: {}}}", self.depth, self.horizontal)
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufReader};
    use crate::day2::calculate_position;

    #[test]
    fn calculate_basic_position() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".as_bytes();
        let reader = BufReader::new(input);
        let pos = calculate_position(reader, 0).unwrap();

        assert_eq!(pos.horizontal, 15);
        assert_eq!(pos.depth, 60);
    }
}
