//! # 2024 day 2 - Red-Nosed Reports
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(PartialEq)]
enum SafetyLevel {
    Safe,
    DampenerSafe,
    Unsafe,
}

struct Report {
    levels: Vec<usize>,
}

impl FromStr for Report {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            levels: s
                .split_whitespace()
                .map(|level| level.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Report {
    fn get_safety_level(&self) -> SafetyLevel {
        fn is_safe(levels: &[usize]) -> bool {
            // Remember the first difference's direction instead of collecting all of them.
            let mut direction = None;
            levels.windows(2).all(|w| {
                let diff = w[1].abs_diff(w[0]);
                if !(1..=3).contains(&diff) {
                    return false;
                }
                let order = w[1].cmp(&w[0]);
                *direction.get_or_insert(order) == order
            })
        }
        if is_safe(&self.levels) {
            SafetyLevel::Safe
        } else {
            for n in 0..self.levels.len() {
                let remove_one: Vec<usize> = self
                    .levels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != n)
                    .map(|(_, level)| *level)
                    .collect();
                if is_safe(&remove_one) {
                    return SafetyLevel::DampenerSafe;
                }
            }
            SafetyLevel::Unsafe
        }
    }
}

struct InputData {
    reports: Vec<Report>,
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
    fn solve(&self) -> (usize, usize) {
        let safety: Vec<SafetyLevel> = self.reports.iter().map(|r| r.get_safety_level()).collect();
        let safe: usize = safety.iter().filter(|s| **s == SafetyLevel::Safe).count();
        let almost_safe: usize = safety
            .iter()
            .filter(|s| **s == SafetyLevel::DampenerSafe)
            .count();
        (safe, safe + almost_safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 2);
        assert_eq!(p2, 4);
    }
}
