mod day1;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let _verbosity = matches.occurrences_of("v");

    match matches.subcommand() {
        Some(("day1", sub_m)) => {
            println!("Day 1 called!");
            println!("Using input file: {}", sub_m.value_of("INPUT").unwrap());
        }
        _ => { /* IDK something? */ }
    }
}
