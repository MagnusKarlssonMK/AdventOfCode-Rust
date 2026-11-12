//! # 2022 day 1 - Calorie Counting
//!
//! Split the input into groups by double line breaks, sum each group and store it in a sorted
//! vector. Then the answer for part 1 is simply the last element in the list, and for part 2 the
//! sum of the last three elements.
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
    elfgroups: Vec<u64>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_group(s: &str) -> Result<u64, AocError> {
            s.lines()
                .map(|elf| elf.parse::<u64>().map_err(AocError::from))
                .sum()
        }
        let mut elfgroups: Vec<_> = s.split("\n\n").map(parse_group).collect::<Result<_, _>>()?;
        elfgroups.sort_unstable();
        Ok(Self { elfgroups })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        *self.elfgroups.last().expect("elf groups cannot be empty")
    }

    fn solve_part2(&self) -> u64 {
        self.elfgroups.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 24000);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 45000);
    }

    #[test]
    fn part1_2_custom_1() {
        // Only two groups -> p2 will only sum those two.
        let solution_data: InputData = "2000\n\n1000".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2000);
        assert_eq!(solution_data.solve_part2(), 3000);
    }
}
