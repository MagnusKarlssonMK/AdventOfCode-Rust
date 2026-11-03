//! # 20xx day xx -
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    data: Vec<String>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: s.lines().map(|line| line.to_string()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        for line in &self.data {
            println!("{}", line);
        }
        1
    }

    fn solve_part2(&self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "test string";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
