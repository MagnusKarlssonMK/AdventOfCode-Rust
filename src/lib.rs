use std::error::Error;
use std::fs;
use std::env;

pub mod year2015;
pub mod year2016;

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
    let filename = cwd.join("AdventOfCode-Input").join(&config.year).join(format!("day{:02}.txt", &config.day));
    let aoc_input = fs::read_to_string(filename)?.trim().to_string();

    // Run solver
    match config.year.as_str() {
        "2015" =>
            match config.day.as_str() {
                "01" => year2015::day01::solve(&aoc_input),
                _ => println!("Day not implemented")
            }
        "2016" =>
            match config.day.as_str() {
                "01" => year2016::day01::solve(&aoc_input),
                _ => println!("Day not implemented")
            }
        _ => println!("Year not implemented")
    }

    Ok(())
}
