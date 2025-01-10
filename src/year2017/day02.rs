//! # 2017 day 2 - Corruption Checksum
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    spreadheet: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            spreadheet: s
                .lines()
                .map(|row| {
                    let mut values: Vec<_> = row
                        .split_whitespace()
                        .map(|nbr| nbr.parse().unwrap())
                        .collect();
                    values.sort_unstable();
                    values
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.spreadheet
            .iter()
            .map(|row| row.last().unwrap() - row.first().unwrap())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.spreadheet
            .iter()
            .map(|row| {
                let mut result: usize = 0;
                for i in 0..row.len() - 1 {
                    for j in i + 1..row.len() {
                        if row[j] % row[i] == 0 {
                            result += row[j] / row[i];
                            break;
                        }
                    }
                }
                result
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = "5 1 9 5\n7 5 3\n2 4 6 8";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 9);
    }
}
