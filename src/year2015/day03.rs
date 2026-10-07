//! # 2015 day 3 - Perfectly Spherical Houses in a Vacuum
use crate::aoc_util::{error::AocError, point::*};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

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
                    '>' => RIGHT,
                    '<' => LEFT,
                    '^' => UP,
                    'v' => DOWN,
                    _ => ORIGIN,
                })
                .collect(),
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
            if count % 2 == 0 {
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
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_official_3() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part2_official_4() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_4).unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_official_3() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_3).unwrap();
        assert_eq!(solution_data.solve_part2(), 11);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }
}
