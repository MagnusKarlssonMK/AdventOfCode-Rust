use advent_of_code_rust::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        eprintln!("Usage: {} <year> <day>", env!("CARGO_PKG_NAME"));
        process::exit(1);
    });

    if let Err(e) = advent_of_code_rust::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
