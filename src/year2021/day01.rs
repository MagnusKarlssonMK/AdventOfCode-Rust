//! # 2021 day 1 - Sonar Sweep
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    measurements: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            measurements: s.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn get_depth(&self, windowsize: usize) -> usize {
        self.measurements
            .windows(windowsize + 1)
            .filter(|w| w[0] < w[windowsize])
            .count()
    }

    fn solve_part1(&self) -> usize {
        self.get_depth(1)
    }

    fn solve_part2(&self) -> usize {
        self.get_depth(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 5);
    }
}
