use std::env;
use std::process;
use advent_of_code_rust::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();

    //let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = advent_of_code_rust::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
