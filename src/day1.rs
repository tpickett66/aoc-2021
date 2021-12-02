use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

pub fn run(input_path: &str, window_size: u64, _verbosity: u64) -> Result<(), Error> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let increase_count = count_increases(reader, window_size as usize);

    println!("Number of times increased: {}", increase_count);

    Ok(())
}

fn count_increases<T: Read>(reader: BufReader<T>, window_size: usize) -> u64 {
    let mut window: VecDeque<u32> = VecDeque::with_capacity(window_size + 1);
    let mut increases = 0;

    for line in reader.lines() {
        let val = line.unwrap().parse::<u32>().unwrap();

        if window.len() > window_size {
            let _ = window.pop_front();
        }

        window.push_back(val);

        if window.len() == window_size + 1 {
            let first_window: u32 = window.range(0..window_size).sum();
            let second_window: u32 = window.range(1..).sum();

            if second_window > first_window {
                increases += 1;
            }
        }
    }
    increases
}

#[cfg(test)]
mod test {
    use std::io::{BufReader};
    use crate::day1::count_increases;

    #[test]
    fn test_count_increases() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".as_bytes();
        let reader = BufReader::new(input);
        let result = count_increases(reader, 1);
        assert_eq!(result, 7)
    }

    #[test]
    fn test_count_increases_windowed() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".as_bytes();
        let reader = BufReader::new(input);
        let result = count_increases(reader, 3);
        assert_eq!(result, 5);
    }
}
