//! # 2023 day 9 - Mirage Maintenance
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    numbers: Vec<Vec<isize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|nbr| nbr.parse().unwrap())
                        .collect()
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> isize {
        self.numbers.iter().map(|line| find_next_number(line)).sum()
    }

    fn solve_part2(&self) -> isize {
        self.numbers
            .iter()
            .map(|line| {
                let r: Vec<isize> = line.iter().rev().copied().collect();
                find_next_number(&r)
            })
            .sum()
    }
}

fn find_next_number(nbrs: &[isize]) -> isize {
    if nbrs.iter().filter(|n| **n != 0).count() == 0 {
        0
    } else {
        let mut nextlevellist = Vec::new();
        for i in 1..nbrs.len() {
            nextlevellist.push(nbrs[i] - nbrs[i - 1]);
        }
        nbrs.last().unwrap() + find_next_number(&nextlevellist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 114);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
