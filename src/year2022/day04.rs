//! # 2022 day 4 - Camp Cleanup
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    assignments: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            assignments: s
                .lines()
                .map(|line| {
                    line.split([',', '-'])
                        .map(|nbr| nbr.parse().unwrap())
                        .collect()
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.assignments
            .iter()
            .map(|a| {
                if (a[0] <= a[2] && a[2] <= a[3] && a[3] <= a[1])
                    || (a[2] <= a[0] && a[0] <= a[1] && a[1] <= a[3])
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.assignments
            .iter()
            .map(|a| if a[0] <= a[3] && a[2] <= a[1] { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
