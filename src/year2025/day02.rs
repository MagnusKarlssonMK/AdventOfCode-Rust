//! # 2025 day 2 - Gift Shop
//!
//! ## Part 1
//!
//! For each range,
//! 1) If the first number in the range is evenly divisible by 2, split it in two and take the
//!    first part (1234 -> 12). If not, start with the next number where it would increase the
//!    number of digits (e.g. 972 -> 1000).
//! 2) Create a new candidate id by combining the number from 1) with itself (12 -> 1212).
//! 3) If that candidate id is bigger than the second value in the range, stop.
//! 4) If that candidate id is bigger than the first value in the range, an invalid value has been
//!    found; add it to the total.
//! 5) Increment the candidate id (12 -> 13) and go back to 2).
//!
//! ## Part 2
//!
//! Pretty much same as part 1, except,
//! * Instead of just splitting in 2 parts, loop over splits in different part sizes, from 2 up to
//!   the number of digits in the second value in the range.
//! * The same number can be found with different part lengths, but should only be counted once. So
//!   the identified invalid numbers are stored in a set and then summarized at the end to get rid
//!   of duplicates.
//!
//! Range endpoints are parsed once into `u64`; digit counts and leading blocks are derived
//! arithmetically instead of repeatedly slicing and re-parsing the original strings.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

/// Number of decimal digits, treating zero as one digit.
fn digits(n: u64) -> u32 {
    1 + n.checked_ilog10().unwrap_or(0)
}

/// Smallest block that can reach `start` when repeated `parts` times.
fn initial_part(start: u64, parts: u32) -> u64 {
    let start_digits = digits(start);
    if start_digits.is_multiple_of(parts) {
        start / 10_u64.pow(start_digits - start_digits / parts)
    } else {
        10_u64.pow(start_digits / parts)
    }
}

/// Repeat `part` `parts` times, padding each copy to `part_digits` digits.
fn repeated(part: u64, parts: u32, part_digits: u32) -> Option<u64> {
    let scale = 10_u64.checked_pow(part_digits)?;
    let mut factor = 1u64;
    let mut result = 0u64;
    for p in 0..parts {
        result = result.checked_add(part.checked_mul(factor)?)?;
        if p + 1 < parts {
            factor = factor.checked_mul(scale)?;
        }
    }
    Some(result)
}

fn get_invalid(start: u64, stop: u64) -> u64 {
    let mut invalid = 0;
    let mut next_part = initial_part(start, 2);
    while let Some(candidate) = repeated(next_part, 2, digits(next_part)) {
        if candidate > stop {
            break;
        }
        if candidate >= start {
            invalid += candidate;
        }
        next_part += 1;
    }
    invalid
}

fn get_multiple_invalid(start: u64, stop: u64) -> u64 {
    let mut invalid = HashSet::new();
    for parts in 2..=digits(stop) {
        let mut next_part = initial_part(start, parts);
        while let Some(candidate) = repeated(next_part, parts, digits(next_part)) {
            if candidate > stop {
                break;
            }
            if candidate >= start {
                invalid.insert(candidate);
            }
            next_part += 1;
        }
    }
    invalid.iter().sum()
}

struct InputData {
    ranges: Vec<(u64, u64)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(',')
            .map(|range| {
                let (start, stop) = range.split_once('-').ctx("range must contain '-'")?;
                if start.is_empty() || stop.is_empty() {
                    return Err(AocError::Invalid(format!("invalid range: {range}")));
                }
                Ok((start.parse()?, stop.parse()?))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { ranges })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.ranges
            .iter()
            .map(|&(start, stop)| get_invalid(start, stop))
            .sum()
    }

    fn solve_part2(&self) -> u64 {
        self.ranges
            .iter()
            .map(|&(start, stop)| get_multiple_invalid(start, stop))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1227775554);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 4174379265);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["", "11", "11-", "-22", "11-22,,33-44"] {
            assert!(
                matches!(input.parse::<InputData>(), Err(AocError::Invalid(_))),
                "expected Invalid for {input:?}"
            );
        }
        assert!(matches!(
            "x-3".parse::<InputData>(),
            Err(AocError::ParseInt(_))
        ));
    }
}
