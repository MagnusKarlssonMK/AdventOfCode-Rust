//! # 2015 day 3 - Perfectly Spherical Houses in a Vacuum
use crate::aoc_util::{error::AocError, point::*};
use std::{collections::HashSet, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct InputData {
    steps: Vec<Point>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            steps: s
                .chars()
                .map(|c| match c {
                    '>' => Ok(RIGHT),
                    '<' => Ok(LEFT),
                    '^' => Ok(UP),
                    'v' => Ok(DOWN),
                    _ => Err(AocError::Invalid(c.to_string())),
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut santa = ORIGIN;
        let mut visited = HashSet::from([santa]);
        for step in &self.steps {
            santa += *step;
            visited.insert(santa);
        }
        visited.len()
    }

    fn solve_part2(&self) -> usize {
        let mut santa = ORIGIN;
        let mut robosanta = ORIGIN;
        let mut visited = HashSet::from([santa]);
        for (count, step) in self.steps.iter().enumerate() {
            if count.is_multiple_of(2) {
                santa += *step;
                visited.insert(santa);
            } else {
                robosanta += *step;
                visited.insert(robosanta);
            }
        }
        visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = ">";
    const TEST_OFFICIAL_2: &str = "^>v<";
    const TEST_OFFICIAL_3: &str = "^v^v^v^v^v";
    const TEST_OFFICIAL_4: &str = "^v";
    const TEST_CUSTOM_1: &str = "^>v<^";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 11);
    }

    #[test]
    fn part2_official_4() {
        let solution_data: InputData = TEST_OFFICIAL_4.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn parser_invalid_char() {
        let err = "v<K>".parse::<InputData>().unwrap_err();
        assert_eq!(err, AocError::Invalid("K".to_string()));
    }
}
