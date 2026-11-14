//! # 2022 day 6 - Tuning Trouble
//!
//! The input is only using lower case ascii characters, so we can store the characters as a vec of
//! u32 bitmasks. Then depending on the window size for each part, we can use the windows iterator
//! to go through the input. For each window, fold the values with a bitwise OR and count the ones
//! of the result and if it is the same as the window size, we found the solution.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const P1_MARKER_SIZE: usize = 4;
const P2_MARKER_SIZE: usize = 14;

struct InputData {
    datastream: Vec<u32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            datastream: s
                .chars()
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        Ok(1 << (c as u32 - 'a' as u32))
                    } else {
                        Err(AocError::Invalid(format!("not a valid char: {c}")))
                    }
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<usize, AocError> {
        self.get_processed_characters(P1_MARKER_SIZE)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        self.get_processed_characters(P2_MARKER_SIZE)
    }

    fn get_processed_characters(&self, start: usize) -> Result<usize, AocError> {
        for (i, w) in self.datastream.windows(start).enumerate() {
            if w.iter().fold(0, |acc, v| acc | v).count_ones() == start as u32 {
                return Ok(i + start);
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_OFFICIAL_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_OFFICIAL_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_OFFICIAL_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_OFFICIAL_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 7);
        assert_eq!(solution_data.solve_part2().unwrap(), 19);
    }

    #[test]
    fn part1_2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 5);
        assert_eq!(solution_data.solve_part2().unwrap(), 23);
    }

    #[test]
    fn part1_2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 6);
        assert_eq!(solution_data.solve_part2().unwrap(), 23);
    }

    #[test]
    fn part1_2_official_4() {
        let solution_data: InputData = TEST_OFFICIAL_4.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 10);
        assert_eq!(solution_data.solve_part2().unwrap(), 29);
    }

    #[test]
    fn part1_2_official_5() {
        let solution_data: InputData = TEST_OFFICIAL_5.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 11);
        assert_eq!(solution_data.solve_part2().unwrap(), 26);
    }

    #[test]
    fn part1_2_custom_1() {
        // Not enough characters to get an answer for part 2
        let solution_data: InputData = "abcabcd".parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 7);
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }

    #[test]
    fn part1_2_custom_2() {
        // Never reaching more than 3 consecutive unique characters
        let solution_data: InputData = "abcabcabcabcabcabcabcabc".parse().unwrap();
        assert_eq!(
            solution_data.solve_part1().unwrap_err(),
            AocError::NoSolution
        );
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }

    #[test]
    fn parser_invalid_input() {
        assert!(matches!(
            "abcDe".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }
}
