use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

pub fn run(_verbosity: u64, input_path: &str) -> Result<(), Error> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    // reader.lines()
    let increase_count = count_increases(reader);

    println!("Number of times increased: {}", increase_count);

    Ok(())
}

fn count_increases<T: Read>(reader: BufReader<T>) -> u64 {
    let mut previous = 0;
    let mut increases = 0;

    for line in reader.lines() {
        let val = line.unwrap().parse::<i32>().unwrap();

        if previous != 0 {
            if val > previous {
                increases += 1;
            }
        }
        previous = val;
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
        let result = count_increases(reader);
        assert_eq!(result, 7)
    }
}
