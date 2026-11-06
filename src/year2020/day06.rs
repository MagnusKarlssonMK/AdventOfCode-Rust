//! # 2020 day 6 - Custom Customs
//!
//! Represents the answer from each passenger as a 26-bit bitmap; one bit for each present
//! character in their answer. Then the answer for part 1 is simply to count the number of ones in
//! the bitwise-or of all answers in the group, and then take the sum of all groups. Similarly for
//! part 2, with the only difference that bitwise-and is used instead.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Group {
    answers: Vec<u32>,
}

impl FromStr for Group {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_passenger(s: &str) -> Result<u32, AocError> {
            s.chars().try_fold(0, |acc, c| {
                if c.is_ascii_lowercase() {
                    Ok(acc | (1 << (c as u8 - b'a') as u32))
                } else {
                    Err(AocError::Invalid(s.to_string()))
                }
            })
        }
        Ok(Self {
            answers: s.lines().map(parse_passenger).collect::<Result<_, _>>()?,
        })
    }
}

impl Group {
    fn get_yes_count_anyone(&self) -> u32 {
        self.answers.iter().fold(0, |acc, &x| acc | x).count_ones()
    }

    fn get_yes_count_everyone(&self) -> u32 {
        self.answers
            .iter()
            .fold(u32::MAX, |acc, &x| acc & x)
            .count_ones()
    }
}

struct InputData {
    forms: Vec<Group>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            forms: s
                .split("\n\n")
                .map(|g| g.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u32 {
        self.forms.iter().map(|f| f.get_yes_count_anyone()).sum()
    }

    fn solve_part2(&self) -> u32 {
        self.forms.iter().map(|f| f.get_yes_count_everyone()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "abcx
abcy
abcz";

    const TEST_OFFICIAL_2: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 11);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }
}
