//! # 2015 day 4 - The Ideal Stocking Stuffer
use crate::aoc_util::error::AocError;
use md5::{Digest, Md5};
use std::error::Error;

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
        Ok(Self { secret_key: s })
    }
}

impl InputData<'_> {
    fn find_lowest_number(&self, prefix: &str) -> usize {
        let mut suffix: usize = 0;
        loop {
            let mut candidate = self.secret_key.to_string();
            candidate.push_str(&suffix.to_string());
            if format!("{:x}", Md5::digest(&candidate)).starts_with(prefix) {
                break;
            }
            suffix += 1;
        }
        suffix
    }

    fn solve_part1(&self) -> usize {
        self.find_lowest_number("00000")
    }

    fn solve_part2(&self) -> usize {
        self.find_lowest_number("000000")
    }
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
