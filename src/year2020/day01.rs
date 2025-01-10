//! # 2020 day 1 - Report Repair
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const TARGET: usize = 2020;

struct InputData {
    reports: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            reports: s.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let nbr_reports = self.reports.len();
        for i in 0..nbr_reports - 1 {
            for j in i + 1..nbr_reports {
                if self.reports[i] + self.reports[j] == TARGET {
                    return self.reports[i] * self.reports[j];
                }
            }
        }
        0
    }

    fn solve_part2(&self) -> usize {
        let nbr_reports = self.reports.len();
        for i in 0..nbr_reports - 2 {
            for j in i + 1..nbr_reports - 1 {
                let pairsum = self.reports[i] + self.reports[j];
                if pairsum < TARGET {
                    for k in j + 1..nbr_reports {
                        if pairsum + self.reports[k] == TARGET {
                            return self.reports[i] * self.reports[j] * self.reports[k];
                        }
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 514579);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 241861950);
    }
}
