//! # 2016 day 20 - Firewall Rules
//!
//! Store the input in a vector of low & high values, sorted based on the low value.
//!
//! ## Part 1
//!
//! Start with a candidate of 0 and walk the sorted ranges: as soon as the candidate
//! falls below a range's low it is not blocked, otherwise it is bumped to just past
//! the range's high.
//!
//! ## Part 2
//!
//! Count the addresses that are not covered by any blocked interval: walk the sorted
//! ranges and add up the gaps between them, including the gap below the first range
//! and the one above the last range.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const MAX_IP: u64 = 2u64.pow(32) - 1;

#[derive(Debug)]
struct InputData {
    block_list: Vec<(u64, u64)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut block_list: Vec<(u64, u64)> = s
            .lines()
            .map(|line| {
                let (low, high) = line.split_once('-').ctx("dash separator")?;
                Ok((low.parse()?, high.parse()?))
            })
            .collect::<Result<Vec<(u64, u64)>, AocError>>()?;
        block_list.sort_unstable();
        Ok(Self { block_list })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        let mut candidate = 0;
        for (low, high) in &self.block_list {
            if candidate < *low {
                break;
            }
            if candidate <= *high {
                candidate = high + 1;
            }
        }
        candidate
    }

    fn solve_part2(&self) -> u64 {
        // Count the addresses that are not covered by any blocked interval.
        let mut allowed_total = 0;
        // The lowest address that is not accounted for yet.
        let mut next_free = 0;
        for (low, high) in &self.block_list {
            if *low > next_free {
                // The addresses between next_free and low - 1 are not blocked
                allowed_total += low - next_free;
                next_free = *low;
            }
            next_free = next_free.max(high + 1);
        }
        // The addresses above the last blocked range are not blocked either
        allowed_total += (MAX_IP + 1).saturating_sub(next_free);
        allowed_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "5-8
0-2
4-7";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_custom_1() {
        assert_eq!(InputData::from_str("0-0").unwrap().solve_part1(), 1);
        assert_eq!(InputData::from_str("4-5").unwrap().solve_part1(), 0);
        assert_eq!(InputData::from_str("0-1\n2-3").unwrap().solve_part1(), 4);
        assert_eq!(InputData::from_str("0-10\n5-6").unwrap().solve_part1(), 11);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 4294967288);
    }

    #[test]
    fn part2_custom_1() {
        assert_eq!(
            InputData::from_str("5-8").unwrap().solve_part2(),
            4294967292
        );
        assert_eq!(
            InputData::from_str("1-1").unwrap().solve_part2(),
            4294967295
        );
    }

    #[test]
    fn parser_missing_separator() {
        let err = InputData::from_str("1234").unwrap_err();
        assert_eq!(err, AocError::Missing("dash separator"));
    }

    #[test]
    fn parser_noninteger() {
        let err = InputData::from_str("1c34-3210").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
        let err = InputData::from_str("1234-3x10").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn empty_block_list() {
        let solution_data = InputData::from_str("").unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 4294967296);
    }

    #[test]
    fn everything_blocked() {
        let solution_data = InputData::from_str("0-4294967295").unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }
}
