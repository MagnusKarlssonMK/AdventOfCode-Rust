//! # 2017 day 15 - Dueling Generators
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1(PART_1_PAIRS).to_string(),
        solution_data.solve_part2(PART_2_PAIRS).to_string(),
    ))
}

const FACTORS: (usize, usize) = (16807, 48271);
const MULTIPLES: (usize, usize) = (4, 8);
const DIVISOR: usize = 2147483647;
const PART_1_PAIRS: usize = 40_000_000;
const PART_2_PAIRS: usize = 5_000_000;

struct InputData {
    start_values: (usize, usize),
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line1, line2) = s.split_once('\n').ctx("lines")?;
        Ok(Self {
            start_values: (
                line1.split_ascii_whitespace().last().ctx("nbr")?.parse()?,
                line2.split_ascii_whitespace().last().ctx("nbr")?.parse()?,
            ),
        })
    }
}

impl InputData {
    fn solve_part1(&self, pairs: usize) -> usize {
        let (mut a, mut b) = self.start_values;
        let mut matches = 0;
        for _ in 0..pairs {
            a = a * FACTORS.0 % DIVISOR;
            b = b * FACTORS.1 % DIVISOR;
            if (a & 0xFFFF) == (b & 0xFFFF) {
                matches += 1;
            }
        }
        matches
    }

    fn solve_part2(&self, pairs: usize) -> usize {
        let (mut a, mut b) = self.start_values;
        let mut matches = 0;
        for _ in 0..pairs {
            a = a * FACTORS.0 % DIVISOR;
            while a % MULTIPLES.0 != 0 {
                a = a * FACTORS.0 % DIVISOR;
            }
            b = b * FACTORS.1 % DIVISOR;
            while b % MULTIPLES.1 != 0 {
                b = b * FACTORS.1 % DIVISOR;
            }
            if (a & 0xFFFF) == (b & 0xFFFF) {
                matches += 1;
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Generator A starts with 65
Generator B starts with 8921";

    const TEST_CUSTOM_1: &str = "Generator A starts with 1
Generator B starts with 1";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(10_000), 1);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(10_000), 0);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(10_000), 2);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(10_000), 1);
    }
}
