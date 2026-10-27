//! # 2017 day 13 - Packet Scanners
//!
//! For part 2, a delay is safe for a scanner if (delay + depth) % cycle != 0, i.e. the scanner is not
//! back at its start when the packet passes. Fold the scanners in one at a time, keeping every delay
//! that is valid modulo the LCM of the scanner periods processed so far, expanded to the next LCM each
//! time.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    math,
};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Scanner {
    depth: usize,
    range: usize,
    cycle: usize,
}

impl FromStr for Scanner {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").ctx("separator")?;
        let range = right.parse()?;
        Ok(Self {
            depth: left.parse()?,
            range,
            cycle: 2 * (range - 1),
        })
    }
}

impl Scanner {
    #[inline]
    fn get_severity(&self) -> usize {
        self.depth * self.range
    }
}

#[derive(Debug)]
struct InputData {
    scanners: Vec<Scanner>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            scanners: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.scanners
            .iter()
            .filter(|s| s.depth % s.cycle == 0)
            .map(|s| s.get_severity())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut current_lcm = 1;
        let mut delays = Vec::from([1]);
        for s in self.scanners.iter() {
            let new_lcm = math::lcm(current_lcm, s.cycle);
            let mut new_delays = Vec::new();
            for extra in (0..new_lcm).step_by(current_lcm) {
                delays
                    .iter()
                    .filter(|delay| (**delay + extra + s.depth) % s.cycle != 0)
                    .for_each(|delay| new_delays.push(*delay + extra));
            }
            current_lcm = new_lcm;
            delays = new_delays;
        }
        delays[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 24);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 10);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str("0: 4").unwrap();
        // Caught at time 0 (severity 0 * 4 = 0); the first safe delay is 1.
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data = InputData::from_str("0: 2\n1: 3").unwrap();
        // Two scanners
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn parser_invalid_input() {
        let err = InputData::from_str("0 4").unwrap_err();
        assert_eq!(err, AocError::Missing("separator"));
        let err = InputData::from_str("b: 4").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
        let err = InputData::from_str("0: f").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
