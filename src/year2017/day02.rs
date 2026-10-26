//! # 2017 day 2 - Corruption Checksum
//!
//! Stores the input in sorted rows of numbers, and then calculates the checksum and result
//! for each row.
//!
//! The puzzle description implies that there won't be any empty rows, and that there will
//! be exactly one evenly divisible pair for each row. This implementation doesn't prevent
//! that, it will simply have empty rows / rows with no evenly divisible pairs contribute
//! with zero, and if any row has multiple evenly divisible pairs, only the first one found
//! will be used.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Row {
    /// The numbers derived from the input row, sorted low to high
    numbers: Vec<u32>,
}

impl FromStr for Row {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<u32> = s
            .split_ascii_whitespace()
            .map(|nbr| nbr.parse())
            .collect::<Result<Vec<_>, _>>()?;
        values.sort_unstable();
        Ok(Row { numbers: values })
    }
}

impl Row {
    /// Largest value - smallest value. Returns 0 if empty.
    fn get_checksum(&self) -> u32 {
        *self.numbers.last().unwrap_or(&0) - *self.numbers.first().unwrap_or(&0)
    }

    /// Returns the result from the first evenly divisible pair found, 0 if none is found.
    fn get_result(&self) -> u32 {
        let mut result = 0;
        'outer: for (i, v1) in self.numbers.iter().enumerate() {
            for v2 in self.numbers.iter().skip(i + 1) {
                if *v2 % *v1 == 0 {
                    result += *v2 / *v1;
                    break 'outer;
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct InputData {
    rows: Vec<Row>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s
                .lines()
                .map(Row::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u32 {
        self.rows.iter().map(|row| row.get_checksum()).sum()
    }

    fn solve_part2(&self) -> u32 {
        self.rows.iter().map(|row| row.get_result()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "5 1 9 5
7 5 3
2 4 6 8";

    const TEST_OFFICIAL_2: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 7);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 9);
    }
}
