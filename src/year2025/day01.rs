//! # 2025 day 1 - Secret Entrance
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
