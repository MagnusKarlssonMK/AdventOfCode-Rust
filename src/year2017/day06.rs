//! # 2017 day 6 - Memory Reallocation
//!
//! Solves both parts in one go by running through the rebalancing steps, recording the results
//! in a hashmap to be able to find when a repeated result occurs. The answer for part 1 is then
//! the number of steps it took to find a repeat, and the answer for part two is the cycle length,
//! i.e the difference between current number of steps and the number of steps it took to find it
//! the first time.
use std::{collections::HashMap, error::Error, str::FromStr};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

fn rebalance(banks: &mut [i32]) {
    let mut maxidx = banks
        .iter()
        .enumerate()
        .min_by_key(|(_, val)| -*val)
        .unwrap()
        .0;
    // Note - need to use min-by and negate the key to get the first index, since
    // max-by returns the last index in case of duplicate max values
    let mut maxval = banks[maxidx];
    let nbrbanks = banks.len();
    banks[maxidx] = 0;
    while maxval > 0 {
        maxidx = (maxidx + 1) % nbrbanks;
        banks[maxidx] += 1;
        maxval -= 1;
    }
}

struct InputData {
    memorybanks: Vec<i32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            memorybanks: s
                .split_ascii_whitespace()
                .map(|nbr| nbr.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&mut self) -> (u32, u32) {
        let mut seen_configs = HashMap::new();
        let mut banks = self.memorybanks.clone();
        let mut steps = 0;
        loop {
            if let Some(v) = seen_configs.get(&banks) {
                let cycles = steps - *v;
                return (steps, cycles);
            } else {
                seen_configs.insert(banks.clone(), steps);
                rebalance(&mut banks);
                steps += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_official_1() {
        let mut solution_data = InputData::from_str("0 2 7 0").unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 5);
        assert_eq!(p2, 4);
    }

    #[test]
    fn part1_2_custom_1() {
        let mut solution_data = InputData::from_str("1").unwrap();
        // A single bank rebalances back to itself immediately.
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1);
        assert_eq!(p2, 1);
    }

    #[test]
    fn part1_2_custom_2() {
        let mut solution_data = InputData::from_str("0 0").unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1);
        assert_eq!(p2, 1);
    }

    #[test]
    fn part1_2_custom_3() {
        let mut solution_data = InputData::from_str("5 0 0").unwrap();
        // The redistributed value exceeds the bank count, wrapping more than once.
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 4);
        assert_eq!(p2, 3);
    }
}
