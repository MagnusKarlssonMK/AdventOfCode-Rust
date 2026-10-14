//! # 2015 day 4 - The Ideal Stocking Stuffer
use crate::aoc_util::error::AocError;
use md5::{Digest, Md5};
use std::{error::Error, fmt::Write};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    secret_key: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            secret_key: s.trim(),
        })
    }
}

impl InputData<'_> {
    /// Finds the lowest suffix whose `md5(secret_key + suffix)` starts with at
    /// least `zeros` zero nibbles (i.e. `4 * zeros` zero bits).
    fn find_lowest_number(&self, zeros: usize) -> usize {
        let mut buffer = String::from(self.secret_key);
        let base_len = buffer.len();
        let mut suffix = 0usize;
        loop {
            buffer.truncate(base_len);
            write!(buffer, "{suffix}").unwrap();
            let digest = Md5::digest(buffer.as_bytes());
            if leading_zero_nibbles(&digest) >= zeros {
                return suffix;
            }
            suffix += 1;
        }
    }

    fn solve_part1(&self) -> usize {
        self.find_lowest_number(5)
    }

    fn solve_part2(&self) -> usize {
        self.find_lowest_number(6)
    }
}

/// Counts the leading zero nibbles (4-bit groups) of a byte slice.
fn leading_zero_nibbles(bytes: &[u8]) -> usize {
    let mut count = 0;
    for &byte in bytes {
        let zeros = (byte.leading_zeros() / 4) as usize;
        count += zeros;
        if zeros < 2 {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Custom made input that reaches the answer in reasonable time
    const TEST_CUSTOM_1: &str = "aaae";

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 589086);
    }

    // Official input, requires too many hashes to run fast enough - disabling by default
    #[test]
    #[ignore = "slow"]
    fn part1_official_1() {
        let solution_data = InputData::try_from("abcdef").unwrap();
        assert_eq!(solution_data.solve_part1(), 609043);
    }

    // Official input, requires too many hashes to run fast enough - disabling by default
    #[test]
    #[ignore = "slow"]
    fn part1_official_2() {
        let solution_data = InputData::try_from("pqrstuv").unwrap();
        assert_eq!(solution_data.solve_part1(), 1048970);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 808543);
    }
}
