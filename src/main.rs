mod day1;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let verbosity = matches.occurrences_of("v");

    match matches.subcommand() {
        Some(("day1", sub_m)) => {
            let input_file = sub_m.value_of("INPUT").unwrap();
            match day1::run(verbosity, input_file) {
                Ok(_) => {},
                Err(err) => { panic!("{}", err) }
            };
        }
        _ => { /* IDK something? */ }
    }
}
