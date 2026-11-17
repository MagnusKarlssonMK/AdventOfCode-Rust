//! # 2023 day 9 - Mirage Maintenance
//!
//! Each history is extrapolated by building successive rows of differences between neighbouring
//! values, stopping when a row is all zeroes. The next value is the last value of each row summed
//! together. Part 2 reuses the same calculation on reversed histories to get the previous value.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    numbers: Vec<Vec<isize>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|nbr| nbr.parse())
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn extrapolate_total(&self, reverse: bool) -> Result<isize, AocError> {
        self.numbers.iter().try_fold(0isize, |total, line| {
            let value = if reverse {
                let reversed: Vec<_> = line.iter().rev().copied().collect();
                find_next_number(&reversed)?
            } else {
                find_next_number(line)?
            };
            total
                .checked_add(value)
                .ok_or_else(|| AocError::Overflow("history total".to_string()))
        })
    }

    fn solve_part1(&self) -> Result<isize, AocError> {
        self.extrapolate_total(false)
    }

    fn solve_part2(&self) -> Result<isize, AocError> {
        self.extrapolate_total(true)
    }
}

fn find_next_number(nbrs: &[isize]) -> Result<isize, AocError> {
    let Some(&last) = nbrs.last() else {
        return Ok(0);
    };
    if nbrs.iter().all(|&n| n == 0) {
        return Ok(0);
    }
    let next_level: Vec<_> = nbrs
        .windows(2)
        .map(|pair| {
            pair[1]
                .checked_sub(pair[0])
                .ok_or_else(|| AocError::Overflow("history difference".to_string()))
        })
        .collect::<Result<_, _>>()?;
    last.checked_add(find_next_number(&next_level)?)
        .ok_or_else(|| AocError::Overflow("history extrapolation".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const TEST_CUSTOM_1: &str = "1 2 3 4 5";
    const TEST_CUSTOM_2: &str = "0 0 0";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 114);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 6);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 0);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 2);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }

    #[test]
    fn single_value_history() {
        let solution_data: InputData = "5".parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 5);
        assert_eq!(solution_data.solve_part2().unwrap(), 5);
    }

    #[test]
    fn difference_overflow() {
        let solution_data: InputData = "-9223372036854775808 1".parse().unwrap();
        assert!(matches!(
            solution_data.solve_part1(),
            Err(AocError::Overflow(_))
        ));
    }

    #[test]
    fn total_overflow() {
        let solution_data: InputData = "0 9223372036854775807\n0 9223372036854775807"
            .parse()
            .unwrap();
        assert!(matches!(
            solution_data.solve_part1(),
            Err(AocError::Overflow(_))
        ));
    }

    #[test]
    fn parser_invalid_number() {
        assert!(matches!(
            "1 x 3".parse::<InputData>(),
            Err(AocError::ParseInt(_))
        ));
    }
}
