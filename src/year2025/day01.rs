//! # 2025 day 1 - Secret Entrance
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    rotations: Vec<i32>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rotations: s
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
        let mut zero_count = 0;
        let mut dial: i32 = 50;
        for v in &self.rotations {
            dial = (dial + v).rem_euclid(100);
            if dial == 0 {
                zero_count += 1;
            }
        }
        zero_count
    }

    fn solve_part2(&self) -> usize {
        let mut zero_count = 0;
        let mut dial: i32 = 50;
        for v in &self.rotations {
            if *v >= 0 {
                zero_count += ((dial + *v) / 100) as usize;
            } else if dial == 0 {
                zero_count += (v.abs() / 100) as usize;
            } else {
                zero_count += ((100 - dial + v.abs()) / 100) as usize;
            }
            dial = (dial + v).rem_euclid(100);
        }
        zero_count
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
