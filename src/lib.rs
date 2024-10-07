use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub year: String,
    pub day: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();

        let year = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get year."),
        };

        let day = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get day."),
        };

        Ok(Config { year, day })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("AdventOfCode-Input").join(config.year).join(format!("day{:02}.txt", config.day));
    //println!("Reading {}", filename.display());

    let aoc_input = fs::read_to_string(filename)?.trim().to_string();

    println!("{}", aoc_input);

    // TBD - call runner function for the given year+day

    Ok(())
}
