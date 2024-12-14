//! # 2024 day 2 - Red-Nosed Reports
use std::cmp::Ordering;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
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

impl Report {
    fn new(input: &str) -> Self {
        Self {
            levels: input
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect(),
        }
    }

    fn get_safety_level(&self) -> SafetyLevel {
        fn is_safe(levels: &[usize]) -> bool {
            let diffs: Vec<(usize, Ordering)> = levels
                .windows(2)
                .map(|w| (w[1].abs_diff(w[0]), w[1].cmp(&w[0])))
                .collect();
            diffs
                .iter()
                .all(|&(v, o)| (1..=3).contains(&v) && o == diffs.first().unwrap().1)
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

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            reports: input.lines().map(Report::new).collect(),
        }
    }

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

    #[test]
    fn parts1_2_example_1() {
        let testdata = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 2);
        assert_eq!(p2, 4);
    }
}
