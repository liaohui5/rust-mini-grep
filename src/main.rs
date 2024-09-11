use std::env;
use std::process;
use minigrep_4_study_rust::*;

fn main() {
    let iter = env::args();

    let config = Config::new(iter).unwrap_or_else(|err| {
        eprintln!("Fail to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }
}
