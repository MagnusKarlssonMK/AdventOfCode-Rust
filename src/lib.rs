use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub mod aoc_util {
    pub mod error;
    pub mod grid;
    pub mod math;
    pub mod point;
    pub mod thread;
}

macro_rules! solvers {
    ( $( $ymod:ident => [ $( $dmod:ident ),* $(,)? ] ),* $(,)? ) => {
        $( pub mod $ymod; )*

        /// Dispatches to the solver for the given `year_key` (`"yearYYYY"`)
        /// and `day_key` (`"dayDD"`, zero-padded).
        fn dispatch(
            year_key: &str,
            day_key: &str,
            input: &str,
        ) -> Result<(String, String), Box<dyn Error>> {
            match year_key {
                $(
                    stringify!($ymod) => match day_key {
                        $( stringify!($dmod) => $ymod::$dmod::solve(input), )*
                        _ => Err("Day not implemented".into()),
                    },
                )*
                _ => Err("Year not implemented".into()),
            }
        }

        /// Returns `Ok(())` if a solver exists for the given keys, or a
        /// descriptive error otherwise.
        fn is_implemented(year_key: &str, day_key: &str) -> Result<(), Box<dyn Error>> {
            match year_key {
                $(
                    stringify!($ymod) => match day_key {
                        $( stringify!($dmod) => Ok(()), )*
                        _ => Err("Day not implemented".into()),
                    },
                )*
                _ => Err("Year not implemented".into()),
            }
        }
    };
}

solvers! {
    year2015 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day19],
    year2016 => [day01, day02, day03, day04, day06, day07, day08, day09],
    year2017 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day19],
    year2018 => [day01, day02, day03],
    year2019 => [day01, day02, day03, day04, day05, day06],
    year2020 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18],
    year2021 => [day01, day02, day03, day04, day05, day06, day07],
    year2022 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15],
    year2023 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21],
    year2024 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25],
    year2025 => [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12],
}

pub struct Config {
    pub year: String,
    pub day: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
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
    let day_num: u8 = config
        .day
        .parse()
        .map_err(|_| format!("Invalid day: {}", config.day))?;
    let year_key = format!("year{}", config.year);
    let day_key = format!("day{day_num:02}");

    is_implemented(&year_key, &day_key)?;

    let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("AdventOfCode-Input")
        .join(&config.year)
        .join(format!("{day_key}.txt"));
    let aoc_input = fs::read_to_string(filename)?
        .trim_end_matches('\n')
        .to_string();

    // Run solver
    let timer = Instant::now();
    let (p1, p2) = dispatch(&year_key, &day_key, &aoc_input)?;
    let t = timer.elapsed().as_micros();
    println!("Part 1: {p1}\nPart 2: {p2}\nCompleted in: {t} μs");

    Ok(())
}
