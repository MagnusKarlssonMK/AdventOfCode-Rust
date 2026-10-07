//! # 2015 day 8 - Matchsticks
//!
//! Scan each line for special characters.
//!
//! ## Part 1
//!
//! Count number of positions that does not represent an actual character.
//!
//! ## Part 2
//!
//! Count the characters that needs to be escaped.
use crate::aoc_util::error::AocError;
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    literals: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            literals: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        let mut count = 0;
        for literal in &self.literals {
            count += 2; // Add 2 for the surrounding double quotes
            let mut i = 1;
            let chars: Vec<_> = literal.chars().collect();
            while i < literal.len() - 1 {
                if chars[i] == '\\' {
                    match chars[i + 1] {
                        '\"' | '\\' => {
                            count += 1;
                            i += 1;
                        }
                        'x' => {
                            count += 3;
                            i += 3;
                        }
                        _ => (),
                    }
                }
                i += 1;
            }
        }
        count
    }

    fn solve_part2(&self) -> usize {
        // +2 for each element since surrounding double quotes will expand by one character each
        self.literals
            .iter()
            .map(|literal| 2 + literal.chars().filter(|c| *c == '\"' || *c == '\\').count())
            .sum()
    }
}

// Too much hassle to get the test input data working with escape characters...
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CUSTOM_1: &str = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";

    const TEST_CUSTOM_2: &str = "\"\\\\x27\"";

    const TEST_CUSTOM_3: &str = "\"\\x27\\\"\"";

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_custom_3() {
        let solution_data = InputData::try_from(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 19);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn part2_custom_3() {
        let solution_data = InputData::try_from(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part2(), 7);
    }
}
