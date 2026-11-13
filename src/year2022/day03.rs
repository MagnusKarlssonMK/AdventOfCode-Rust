//! # 2022 day 3 - Rucksack Reorganization
//!
//! Item priorities are 1-52, so each compartment is represented as a `u64` bitmask where priority
//! `p` maps to bit `p - 1`. Finding common items then becomes a single bitwise AND, and a shared
//! priority is recovered with `trailing_zeros()`.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

/// Returns one bit per priority: priority 1 -> bit 0, ..., priority 52 -> bit 51.
fn mask(priorities: &[u32]) -> u64 {
    priorities
        .iter()
        .fold(0, |mask, &p| mask | (1u64 << (p - 1)))
}

/// Extracts the single shared priority from a mask, rejecting empty or ambiguous results.
fn single_priority(mask: u64, context: &str) -> Result<u32, AocError> {
    match mask.count_ones() {
        1 => Ok(mask.trailing_zeros() + 1),
        0 => Err(AocError::Invalid(format!("no common item in {context}"))),
        n => Err(AocError::Invalid(format!("{n} common items in {context}"))),
    }
}

struct InputData {
    /// Left and right compartment bitmasks for each rucksack.
    rucksacks: Vec<(u64, u64)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_priority(c: char) -> Result<u32, AocError> {
            if c.is_ascii_uppercase() {
                Ok(27 + (c as u32 - 'A' as u32))
            } else if c.is_ascii_lowercase() {
                Ok(1 + (c as u32 - 'a' as u32))
            } else {
                Err(AocError::Invalid(format!("illegal char: {c}")))
            }
        }
        let rucksacks = s
            .lines()
            .map(|line| {
                let priorities: Vec<u32> =
                    line.chars().map(get_priority).collect::<Result<_, _>>()?;
                if priorities.is_empty() || !priorities.len().is_multiple_of(2) {
                    return Err(AocError::Invalid(format!(
                        "rucksack must have a non-zero even length: {line:?}"
                    )));
                }
                let (left, right) = priorities.split_at(priorities.len() / 2);
                Ok((mask(left), mask(right)))
            })
            .collect::<Result<_, AocError>>()?;
        Ok(Self { rucksacks })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<u32, AocError> {
        self.rucksacks
            .iter()
            .enumerate()
            .try_fold(0u32, |total, (i, (left, right))| {
                Ok(total + single_priority(left & right, &format!("rucksack {i}"))?)
            })
    }

    fn solve_part2(&self) -> Result<u32, AocError> {
        if !self.rucksacks.len().is_multiple_of(3) {
            return Err(AocError::Invalid(
                "rucksack count is not a multiple of 3".to_string(),
            ));
        }
        self.rucksacks
            .as_chunks::<3>()
            .0
            .iter()
            .enumerate()
            .try_fold(0u32, |total, (i, group)| {
                let common = group
                    .iter()
                    .fold(u64::MAX, |common, (left, right)| common & (left | right));
                Ok(total + single_priority(common, &format!("group {i}"))?)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 157);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 70);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["asDF#gTEr", "abc", "ab\n\ncd"] {
            assert!(
                matches!(input.parse::<InputData>(), Err(AocError::Invalid(_))),
                "expected {input:?} to be rejected"
            );
        }
    }

    #[test]
    fn part1_requires_exactly_one_common_item() {
        // No shared item between the compartments.
        assert!(matches!(
            "ab".parse::<InputData>().unwrap().solve_part1(),
            Err(AocError::Invalid(_))
        ));
        // Both 'a' and 'b' are shared.
        assert!(matches!(
            "abab".parse::<InputData>().unwrap().solve_part1(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn part2_requires_complete_groups() {
        assert!(matches!(
            "aa\nbb\ncc\ndd".parse::<InputData>().unwrap().solve_part2(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn part2_requires_exactly_one_common_item() {
        // No item shared by all three rucksacks.
        assert!(matches!(
            "aa\nbb\ncc".parse::<InputData>().unwrap().solve_part2(),
            Err(AocError::Invalid(_))
        ));
        // Both 'a' and 'b' are shared by all three rucksacks.
        assert!(matches!(
            "abab\nabab\nabab"
                .parse::<InputData>()
                .unwrap()
                .solve_part2(),
            Err(AocError::Invalid(_))
        ));
    }
}
