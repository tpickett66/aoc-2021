mod day1;
mod day2;
mod util;
mod day3;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let verbosity = matches.occurrences_of("verbose");
    let input_file = matches.value_of("INPUT").unwrap();

    match matches.subcommand() {
        Some(("day1", sub_m)) => {
            let window_size = sub_m.value_of("window").unwrap().parse::<u64>().unwrap();
            match day1::run(input_file, window_size, verbosity) {
                Ok(_) => {},
                Err(err) => { panic!("{}", err) }
            };
        }
        Some(("day2", _sub_m)) => {
            match day2::run(input_file, verbosity) {
                Ok(_) => {},
                Err(err) => { panic!("{}", err) }
            };
        }
        Some(("day3", _sub_m)) => {
            match day3::run(input_file, verbosity) {
                Ok(_) => {},
                Err(err) => { panic!("{}", err) }
            };
        }
        _ => { /* IDK something? */ }
    }
}
