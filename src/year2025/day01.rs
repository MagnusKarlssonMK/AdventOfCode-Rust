//! # 2025 day 1 - Secret Entrance
//!
//! ## Part 1
//! * Store the input as a vector of signed integer values.
//! * Then for all input values, use modulo operation to keep updating the dial value, but we need to
//!   use the rem_euclid function for correct result when the sum of the dial and the next value is negative.
//! * Increment counter whenever the dial value is zero.
//!
//! ## Part 2
//! For all input values,
//! * If the input value is positive, increment the counter with (dial + input value) / 100, i.e.
//!   the number of times the dial will wrap around.
//! * Else if the dial is currently zero, increment the counter with the abs(input value) / 100. We
//!   need to handle this case separately to avoid counting the current zero dial value twice.
//! * Else, the input value is negative, so we need to "count backwards"; increment the counter
//!   with (100 - dial + abs(input value)) / 100.
//! * Then update the dial with the same modulo operation as in part 1.
//!
//! A possible alternative solution for part 2 could be to keep track of the current direction instead of
//! working with signed integers, and flip the dial around whenever the direction changes. That way, all
//! changes to the dial will be in the positive direction, i.e. there would be no need to handle
//! "left" (negative) values separately.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Lock {
    dial: i32,
}

const LOCK_SIZE: i32 = 100;

impl Lock {
    fn new() -> Self {
        Self { dial: 50 }
    }

    fn rotate_and_count(&mut self, value: i32) -> usize {
        self.dial = (self.dial + value).rem_euclid(LOCK_SIZE);
        if self.dial == 0 { 1 } else { 0 }
    }

    fn rotate_and_count_any(&mut self, value: i32) -> usize {
        let count = if value >= 0 {
            ((self.dial + value) / LOCK_SIZE) as usize
        } else if self.dial == 0 {
            (value.abs() / LOCK_SIZE) as usize
        } else {
            ((LOCK_SIZE - self.dial + value.abs()) / LOCK_SIZE) as usize
        };
        self.dial = (self.dial + value).rem_euclid(LOCK_SIZE);
        count
    }
}

struct InputData {
    sequence: Vec<i32>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sequence: s
                .lines()
                .map(|line| {
                    let (direction, value) = line.split_at(1);
                    let value: i32 = value.parse().unwrap();
                    match direction {
                        "L" => -value,
                        "R" => value,
                        _ => panic!(),
                    }
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut lock = Lock::new();
        self.sequence
            .iter()
            .map(|v| lock.rotate_and_count(*v))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut lock = Lock::new();
        self.sequence
            .iter()
            .map(|v| lock.rotate_and_count_any(*v))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }
}
