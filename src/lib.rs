use std::env;
use std::error::Error;
use std::fs;
use std::time::Instant;

pub mod aoc_util {
    pub mod grid;
    pub mod point;
    pub mod thread;
}

pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;
pub mod year2024;

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
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("AdventOfCode-Input")
        .join(&config.year)
        .join(format!("day{:02}.txt", &config.day));
    let aoc_input = fs::read_to_string(filename)?.trim().to_string();

    // Run solver
    let timer = Instant::now();
    match config.year.as_str() {
        "2015" => match config.day.as_str() {
            "01" => year2015::day01::solve(&aoc_input),
            "02" => year2015::day02::solve(&aoc_input),
            "03" => year2015::day03::solve(&aoc_input),
            "04" => year2015::day04::solve(&aoc_input),
            "05" => year2015::day05::solve(&aoc_input),
            "06" => year2015::day06::solve(&aoc_input),
            "19" => year2015::day19::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2016" => match config.day.as_str() {
            "01" => year2016::day01::solve(&aoc_input),
            "03" => year2016::day03::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2017" => match config.day.as_str() {
            "01" => year2017::day01::solve(&aoc_input),
            "02" => year2017::day02::solve(&aoc_input),
            "03" => year2017::day03::solve(&aoc_input),
            "04" => year2017::day04::solve(&aoc_input),
            "05" => year2017::day05::solve(&aoc_input),
            "06" => year2017::day06::solve(&aoc_input),
            "07" => year2017::day07::solve(&aoc_input),
            "08" => year2017::day08::solve(&aoc_input),
            "09" => year2017::day09::solve(&aoc_input),
            "10" => year2017::day10::solve(&aoc_input),
            "11" => year2017::day11::solve(&aoc_input),
            "12" => year2017::day12::solve(&aoc_input),
            "13" => year2017::day13::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2018" => match config.day.as_str() {
            "01" => year2018::day01::solve(&aoc_input),
            "02" => year2018::day02::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2019" => match config.day.as_str() {
            "01" => year2019::day01::solve(&aoc_input),
            "02" => year2019::day02::solve(&aoc_input),
            "03" => year2019::day03::solve(&aoc_input),
            "04" => year2019::day04::solve(&aoc_input),
            "05" => year2019::day05::solve(&aoc_input),
            "06" => year2019::day06::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2020" => match config.day.as_str() {
            "01" => year2020::day01::solve(&aoc_input),
            "02" => year2020::day02::solve(&aoc_input),
            "03" => year2020::day03::solve(&aoc_input),
            "06" => year2020::day06::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2021" => match config.day.as_str() {
            "01" => year2021::day01::solve(&aoc_input),
            "02" => year2021::day02::solve(&aoc_input),
            "03" => year2021::day03::solve(&aoc_input),
            "04" => year2021::day04::solve(&aoc_input),
            "05" => year2021::day05::solve(&aoc_input),
            "06" => year2021::day06::solve(&aoc_input),
            "07" => year2021::day07::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2022" => match config.day.as_str() {
            "01" => year2022::day01::solve(&aoc_input),
            "02" => year2022::day02::solve(&aoc_input),
            "03" => year2022::day03::solve(&aoc_input),
            "04" => year2022::day04::solve(&aoc_input),
            "05" => year2022::day05::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2023" => match config.day.as_str() {
            "01" => year2023::day01::solve(&aoc_input),
            "02" => year2023::day02::solve(&aoc_input),
            "03" => year2023::day03::solve(&aoc_input),
            "04" => year2023::day04::solve(&aoc_input),
            "06" => year2023::day06::solve(&aoc_input),
            "09" => year2023::day09::solve(&aoc_input),
            "11" => year2023::day11::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        "2024" => match config.day.as_str() {
            "01" => year2024::day01::solve(&aoc_input),
            "02" => year2024::day02::solve(&aoc_input),
            "03" => year2024::day03::solve(&aoc_input),
            "04" => year2024::day04::solve(&aoc_input),
            "05" => year2024::day05::solve(&aoc_input),
            "06" => year2024::day06::solve(&aoc_input),
            "07" => year2024::day07::solve(&aoc_input),
            "08" => year2024::day08::solve(&aoc_input),
            "09" => year2024::day09::solve(&aoc_input),
            "10" => year2024::day10::solve(&aoc_input),
            "11" => year2024::day11::solve(&aoc_input),
            "12" => year2024::day12::solve(&aoc_input),
            "13" => year2024::day13::solve(&aoc_input),
            "14" => year2024::day14::solve(&aoc_input),
            "15" => year2024::day15::solve(&aoc_input),
            "16" => year2024::day16::solve(&aoc_input),
            "17" => year2024::day17::solve(&aoc_input),
            "18" => year2024::day18::solve(&aoc_input),
            "19" => year2024::day19::solve(&aoc_input),
            "20" => year2024::day20::solve(&aoc_input),
            "21" => year2024::day21::solve(&aoc_input),
            "22" => year2024::day22::solve(&aoc_input),
            "23" => year2024::day23::solve(&aoc_input),
            "24" => year2024::day24::solve(&aoc_input),
            "25" => year2024::day25::solve(&aoc_input),
            _ => println!("Day not implemented"),
        },
        _ => println!("Year not implemented"),
    }
    println!("Completed in: {} μs", timer.elapsed().as_micros());

    Ok(())
}
