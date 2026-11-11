//! # 2021 day 3 - Binary Diagnostic
//!
//! Stores input values as binary values together with the bit width. Then for part 1 it's simply a
//! matter of calculating the gamma and epsilon ratings for all those values and multiplying them
//! to get the result. For part 2, the lists of values to consider need to be filtered down
//! according to the gamma and epsilon ratings one bit at a time, starting at the most significant
//! bit. The answer is then given by the final remaining number for the oxygen (gamma based) and
//! scrubber (epsilon based) ratings.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

/// Returns whether the most common bit at `mask` is 1; ties resolve to 1 (gamma wins).
#[inline]
fn most_common_bit_is_one(numbers: &[u32], mask: u32) -> bool {
    let ones = numbers.iter().filter(|n| *n & mask != 0).count();
    2 * ones >= numbers.len()
}

struct InputData {
    width: usize,
    numbers: Vec<u32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nbrs: Vec<_> = s.lines().collect();
        // Width should be the same over all numbers; taking the width of the first number and
        // validate that it's within bounds and that the rest are the same.
        let first = nbrs.first().unwrap_or(&"");
        let width = first.len();
        if width == 0 || width > u32::BITS as usize {
            return Err(AocError::Invalid(format!("bit width {width}")));
        }
        if let Some(line) = nbrs.iter().find(|line| line.len() != width) {
            return Err(AocError::Invalid(format!(
                "line width {} differs from {width}",
                line.len()
            )));
        }
        Ok(Self {
            width,
            numbers: nbrs
                .iter()
                .map(|n| u32::from_str_radix(n, 2))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        let mut gamma = 0;
        let mut epsilon = 0;
        for mask in (0..self.width).map(|bit| 1u32 << bit) {
            if most_common_bit_is_one(&self.numbers, mask) {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }
        }
        u64::from(gamma) * u64::from(epsilon)
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let mut oxygen_candidates = self.numbers.to_vec();
        let mut scrubber_candidates = self.numbers.to_vec();

        for mask in (0..self.width).rev().map(|bit| 1u32 << bit) {
            if oxygen_candidates.len() > 1 {
                let gamma_bit_is_one = most_common_bit_is_one(&oxygen_candidates, mask);
                oxygen_candidates.retain(|c| (*c & mask != 0) == gamma_bit_is_one);
            }
            if scrubber_candidates.len() > 1 {
                let epsilon_bit_is_one = !most_common_bit_is_one(&scrubber_candidates, mask);
                scrubber_candidates.retain(|c| (*c & mask != 0) == epsilon_bit_is_one);
            }
        }
        if oxygen_candidates.len() == 1 && scrubber_candidates.len() == 1 {
            Ok(u64::from(oxygen_candidates[0]) * u64::from(scrubber_candidates[0]))
        } else {
            Err(AocError::NoSolution)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    /// Four three-bit numbers with an even split at bit 1, so the tie-breaking direction decides both answers.
    const TEST_CUSTOM_1: &str = "010\n110\n101\n100";

    /// Two equal numbers never reduces down to a single number, so no solution is possible for part 2.
    const TEST_CUSTOM_2: &str = "011\n011";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 198);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 230);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
        assert_eq!(solution_data.solve_part2().unwrap(), 10);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn parser_invalid_input() {
        assert!("".parse::<InputData>().is_err()); // no lines
        assert!("01\n1".parse::<InputData>().is_err()); // width mismatch
        assert!("0".repeat(33).parse::<InputData>().is_err()); // width > u32::BITS
        assert!("012".parse::<InputData>().is_err()); // non-binary digit
    }
}
