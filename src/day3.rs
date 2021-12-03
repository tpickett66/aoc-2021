use crate::util;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};


pub fn run(reader: BufReader<File>, verbosity: u64) -> util::Result<()> {
    let reader = reader;

    let (gamma, bits) = calculate_gamma(reader, verbosity).unwrap();
    let epsilon = calculate_epsilon(gamma, bits);
    let rate = gamma * epsilon;

    println!("gamma:   {} ({:#016b})", gamma, gamma);
    println!("epsilon: {} ({:#016b})", epsilon, epsilon);
    println!("rate:    {}", rate);

    Ok(())
}

fn calculate_gamma<T: Read>(reader: BufReader<T>, verbosity: u64) -> util::Result<(u64, usize)> {
    let mut gamma= 0;
    let mut ones: Vec<u64> = Vec::new();
    let mut zeros: Vec<u64> = Vec::new();
    let mut bits = 0;

    for (line_no, line_res) in reader.lines().enumerate() {
        let line = line_res.unwrap();

        if line_no == 0 {
            bits = line.len();
            ones.resize(bits, 0);
            zeros.resize(bits, 0);
        }

        if verbosity >= 3 {
            println!("Got line: '{}'", line);
        }

        for (pos, char) in line.chars().enumerate() {
            match char {
                '1' => { ones[pos] += 1 }
                '0' => { zeros[pos] += 1}
                _ => { panic!("unexpected char value '{}'", char) }
            }
        }
    }

    for (pos, (ones_count, zeros_count)) in ones.iter().zip(zeros.iter()).enumerate() {
        if ones_count > zeros_count {
            let increment = 1 << ((bits - 1) - pos);
            gamma += increment;
        }
    }

    if verbosity >= 2 {
        println!("bits: {}", bits);
    }
    Ok((gamma, bits))
}

fn calculate_epsilon(gamma: u64, bits: usize) -> u64 {
    let mask = (1 << bits) - 1;
    let eps = gamma ^ mask;

    eps
}

#[cfg(test)]
mod test {
    use std::io::BufReader;
    use crate::day3::{calculate_epsilon, calculate_gamma};

    #[test]
    fn test_calculate_gamma() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        let reader = BufReader::new(input);

        let (gamma, bits) = calculate_gamma(reader, 3).unwrap();
        assert_eq!(gamma, 22);
        assert_eq!(bits, 5);
    }

    #[test]
    fn test_calculate_epsilon() {
        let epsilon = calculate_epsilon(22, 5);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn test_calculate_combined_value() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        let reader = BufReader::new(input);

        let (gamma, bits) = calculate_gamma(reader, 3).unwrap();
        let epsilon = calculate_epsilon(gamma, bits);
        let combined_value = gamma * epsilon;
        assert_eq!(combined_value, 198);
    }
}
