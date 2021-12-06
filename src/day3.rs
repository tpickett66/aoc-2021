use std::fmt::{Display, Formatter};
use crate::util;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub struct DiagnosticAnalysis {
    gamma: u64,
    epsilon: u64,
    oxygen: u64,
    co2: u64
}

impl DiagnosticAnalysis {
    pub fn power_rating(&self) -> u64 {
        self.gamma * self.epsilon
    }

    pub fn life_support_rating(&self) -> u64 {
        self.oxygen * self.co2
    }
}

impl Default for DiagnosticAnalysis {
    fn default() -> Self {
        DiagnosticAnalysis {
            gamma: 0,
            epsilon: 0,
            oxygen: 0,
            co2: 0
        }
    }
}

impl Display for DiagnosticAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "=== Power Rating ==\nGamma:   {}\nEpsilon: {}\nOverall: {}\n\n=== Life Support ===\nOxygen:  {}\nCO2:     {}\nOverall: {}",
            self.gamma, self.epsilon, self.power_rating(),
            self.oxygen, self.co2, self.life_support_rating()
        )
    }
}

pub fn run(reader: BufReader<File>, verbosity: u64) -> util::Result<DiagnosticAnalysis> {
    let mut analysis = DiagnosticAnalysis::default();

    let (gamma, bits, lines) = calculate_gamma(reader, verbosity).unwrap();
    let epsilon = calculate_epsilon(gamma, bits);

    analysis.gamma = gamma;
    analysis.epsilon = epsilon;
    analysis.oxygen = find_oxygen_value(&lines, bits)?;
    analysis.co2 = find_co2_value(&lines, bits)?;

    Ok(analysis)
}

fn calculate_gamma<T: Read>(reader: BufReader<T>, verbosity: u64) -> util::Result<(u64, usize, Vec<u64>)> {
    let mut values = Vec::<u64>::new();
    let mut gamma= 0;
    let mut ones: Vec<u64> = Vec::new();
    let mut zeros: Vec<u64> = Vec::new();
    let mut bits = 0;

    for (line_no, line_res) in reader.lines().enumerate() {
        let line = line_res.unwrap();
        let line_value = u64::from_str_radix(line.as_str(), 2).unwrap();
        values.push(line_value);

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
    Ok((gamma, bits, values))
}

fn calculate_epsilon(gamma: u64, bits: usize) -> u64 {
    let mask = (1 << bits) - 1;
    let eps = gamma ^ mask;

    eps
}

// This is super ugly but it works :-/
fn find_oxygen_value(input_values: &Vec<u64>, starting_bit: usize) -> util::Result<u64> {
    let mut remaining_values = input_values.clone();

    for bit in (0..starting_bit).rev() {
        let mask = 1<<bit;
        let mut ones = Vec::<u64>::new();
        let mut zeros = Vec::<u64>::new();

        for val in remaining_values {
            if val & mask == mask {
                ones.push(val);
            } else {
                zeros.push(val);
            }
        }

        if ones.len() > zeros.len() {
            remaining_values = ones;
        } else {
            remaining_values = zeros;
        }

        if remaining_values.len() == 1 {
            return Ok(remaining_values[0])
        } else if remaining_values.len() == 2 {
            println!("{:?}", remaining_values);
            let next_mask = 1<<bit-1;
            let val = remaining_values.iter().filter(|&&v| v & next_mask == next_mask).map(|&v| v).collect::<Vec<u64>>()[0];
            return Ok(val)
        }
    }

    return Err(util::Error::new("Search exhausted input without finding a match!"))
}

// So much duplication but I really want to move on.
fn find_co2_value(input_values: &Vec<u64>, starting_bit: usize) -> util::Result<u64> {
    let mut remaining_values = input_values.clone();

    for bit in (0..starting_bit).rev() {
        let mask = 1<<bit;
        let mut ones = Vec::<u64>::new();
        let mut zeros = Vec::<u64>::new();

        for val in remaining_values {
            if val & mask == mask {
                ones.push(val);
            } else {
                zeros.push(val);
            }
        }

        if zeros.len() > ones.len() {
            remaining_values = ones;
        } else {
            remaining_values = zeros;
        }

        if remaining_values.len() == 1 {
            return Ok(remaining_values[0])
        } else if remaining_values.len() == 2 {
            println!("{:?}", remaining_values);
            let next_mask = 1<<bit-1;
            let val = remaining_values.iter().filter(|&&v| v & next_mask == 0).map(|&v| v).collect::<Vec<u64>>()[0];
            return Ok(val)
        }
    }
    Err(util::Error::new("Search exhausted input without finding a match!"))
}

#[cfg(test)]
mod test {
    use std::io::BufReader;
    use crate::day3::{calculate_epsilon, calculate_gamma, find_oxygen_value, find_co2_value};

    #[test]
    fn test_calculate_gamma() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        let reader = BufReader::new(input);

        let (gamma, bits, _) = calculate_gamma(reader, 3).unwrap();
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

        let (gamma, bits, _) = calculate_gamma(reader, 3).unwrap();
        let epsilon = calculate_epsilon(gamma, bits);
        let combined_value = gamma * epsilon;
        assert_eq!(combined_value, 198);
    }
    
    #[test]
    fn test_find_oxygen_value() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        let reader = BufReader::new(input);

        let (_gamma, bits, lines) = calculate_gamma(reader, 3).unwrap();

        let result = find_oxygen_value(&lines, bits).unwrap();

        assert_eq!(result, 23)
    }

    #[test]
    fn test_find_co2_value() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        let reader = BufReader::new(input);

        let (_gamma, bits, lines) = calculate_gamma(reader, 3).unwrap();

        let result = find_co2_value(&lines, bits).unwrap();

        assert_eq!(result, 10)
    }
}
