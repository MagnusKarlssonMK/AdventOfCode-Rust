//! # 2020 day 5 - Binary Boarding
//!
//! A boarding pass code maps directly onto its seat id in binary: F and L are 0 while B and R are
//! 1, so translating the letters and parsing the result as a binary 10-bit number gives the id.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    boardingpasses: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boardingpasses: Vec<_> = s.lines().map(decode_seat_id).collect::<Result<_, _>>()?;
        boardingpasses.sort_unstable();
        Ok(Self { boardingpasses })
    }
}

fn decode_seat_id(s: &str) -> Result<usize, AocError> {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            'B' | 'R' => Ok(1 << i),
            'F' | 'L' => Ok(0),
            _ => Err(AocError::Invalid(s.to_string())),
        })
        .sum::<Result<_, _>>()
}

impl InputData {
    fn solve_part1(&self) -> Result<usize, AocError> {
        self.boardingpasses
            .last()
            .cloned()
            .ok_or(AocError::NoSolution)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        for w in self.boardingpasses.windows(2) {
            if w[1] == w[0] + 2 {
                return Ok(w[0] + 1);
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CUSTOM_1: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
BBFFBBFRRL";

    /// Input with no answer for part 2
    const TEST_CUSTOM_2: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 567);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = "FFFBBBFRRR".parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 119);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = "BBFFBBFRLL".parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 820);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 821);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }
}
