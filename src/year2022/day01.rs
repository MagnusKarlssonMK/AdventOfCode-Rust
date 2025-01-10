//! # 2022 day 1 - Calorie Counting
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    elfgroups: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups: Vec<usize> = s
            .split("\n\n")
            .map(|group| group.lines().map(|elf| elf.parse::<usize>().unwrap()).sum())
            .collect();
        groups.sort_unstable();
        Ok(Self { elfgroups: groups })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        *self.elfgroups.last().unwrap()
    }

    fn solve_part2(&self) -> usize {
        self.elfgroups.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 24000);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 45000);
    }
}
