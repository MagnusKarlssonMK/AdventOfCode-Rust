//! # 2020 day 2 - Password Philosophy
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Password {
    nbr_range: (usize, usize),
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            [nbr_range, letter, password] => {
                let (r1, r2) = nbr_range.split_once('-').ctx("dash separator")?;
                let letter = letter.chars().next().ctx("letter")?;
                Ok(Self {
                    nbr_range: (r1.parse()?, r2.parse()?),
                    letter,
                    password: password.to_string(),
                })
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Password {
    fn is_valid_old_policy(&self) -> bool {
        (self.nbr_range.0..=self.nbr_range.1)
            .contains(&self.password.chars().filter(|c| *c == self.letter).count())
    }

    fn is_valid_new_policy(&self) -> bool {
        let first = self.password.chars().nth(self.nbr_range.0 - 1) == Some(self.letter);
        let second = self.password.chars().nth(self.nbr_range.1 - 1) == Some(self.letter);
        first ^ second
    }
}

struct InputData {
    passwords: Vec<Password>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            passwords: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.passwords
            .iter()
            .filter(|p| p.is_valid_old_policy())
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.passwords
            .iter()
            .filter(|p| p.is_valid_new_policy())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    /// Passwords shorter than the requested positions must not count out-of-range positions.
    const TEST_CUSTOM_1: &str = "1-5 a: ab
3-4 a: ab";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }
}
