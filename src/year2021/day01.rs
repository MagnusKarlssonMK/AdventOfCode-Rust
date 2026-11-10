//! # 2021 day 1 - Sonar Sweep
//!
//! For part 2, we actually only need to compare the first element of the first window with the
//! last element of the second window, since all the elements in between are shared by both. So by
//! making use of that logic, the answer for both part 1 and 2 can be calculated with one function,
//! taking the window length as an input.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    measurements: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            measurements: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn get_depth(&self, windowsize: usize) -> usize {
        self.measurements
            .windows(windowsize + 1)
            .filter(|w| w[0] < w[windowsize])
            .count()
    }

    fn solve_part1(&self) -> usize {
        self.get_depth(1)
    }

    fn solve_part2(&self) -> usize {
        self.get_depth(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 5);
    }
}
