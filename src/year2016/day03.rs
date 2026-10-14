//! # 2016 day 3 - Squares With Three Sides
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct InputData {
    sides: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sides: s
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.sides
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|s| validate_triangle(*s))
            .count()
    }

    fn solve_part2(&self) -> usize {
        let column_wise: Vec<usize> = self
            .sides
            .iter()
            .step_by(3)
            .copied()
            .chain(
                self.sides
                    .iter()
                    .skip(1)
                    .step_by(3)
                    .copied()
                    .chain(self.sides.iter().skip(2).step_by(3).copied()),
            )
            .collect();
        column_wise
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|s| validate_triangle(*s))
            .count()
    }
}

#[inline]
fn validate_triangle(sides: &[usize]) -> bool {
    sides[0] + sides[1] > sides[2]
        && sides[0] + sides[2] > sides[1]
        && sides[1] + sides[2] > sides[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str("9 1 1").unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::from_str("5 5 10").unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_custom_3() {
        let solution_data =
            InputData::from_str("883  357  185\n572   189 424\n842 206 272\n").unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str("100 1 1\n100 2 2\n100 3 3").unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn parse_non_integers() {
        let testdata = "5 1S 25";
        let err = InputData::from_str(testdata).unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
