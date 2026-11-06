//! # 2020 day 1 - Report Repair
//!
//! Brute forcing all combinations until we find the match. This is quite inefficient, but the
//! input is small enough that it still pays off in performance.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const TARGET: usize = 2020;

struct InputData {
    reports: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            reports: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<usize, AocError> {
        let nbr_reports = self.reports.len();
        for i in 0..nbr_reports - 1 {
            for j in i + 1..nbr_reports {
                if self.reports[i] + self.reports[j] == TARGET {
                    return Ok(self.reports[i] * self.reports[j]);
                }
            }
        }
        Err(AocError::NoSolution)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let nbr_reports = self.reports.len();
        for i in 0..nbr_reports - 2 {
            for j in i + 1..nbr_reports - 1 {
                let pairsum = self.reports[i] + self.reports[j];
                if pairsum < TARGET {
                    for k in j + 1..nbr_reports {
                        if pairsum + self.reports[k] == TARGET {
                            return Ok(self.reports[i] * self.reports[j] * self.reports[k]);
                        }
                    }
                }
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "1721\n979\n366\n299\n675\n1456";

    /// Input with no match for either part
    const TEST_CUSTOM_1: &str = "1900\n81\n82";

    /// Input with a match for part 1 but none for part 2
    const TEST_CUSTOM_2: &str = "1990\n30\n40";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 514579);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 241861950);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
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
    fn part1_2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 1990 * 30);
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }
}
