//! # 2017 day 13 - Packet Scanners
use crate::aoc_util::math;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Scanner {
    depth: usize,
    range: usize,
    cycle: usize,
}

impl FromStr for Scanner {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let range = right.parse().unwrap();
        Ok(Self {
            depth: left.parse().unwrap(),
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

struct InputData {
    scanners: Vec<Scanner>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            scanners: s
                .lines()
                .map(|line| Scanner::from_str(line).unwrap())
                .collect(),
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

    const TEST_DATA: &str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 24);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 10);
    }
}
