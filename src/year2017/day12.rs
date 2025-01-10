//! # 2017 day 12 - Digital Plumber
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    numbers: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|n| n.trim_end_matches(',').parse().ok())
                        .skip(1)
                        .collect()
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let size = self.numbers.len();
        let mut seen = vec![false; size];
        let mut program_groups = Vec::new();
        (0..size).for_each(|i| {
            if !seen[i] {
                seen[i] = true;
                program_groups.push(self.search_programs(&mut seen, i));
            }
        });
        (program_groups[0], program_groups.len())
    }

    fn search_programs(&self, seen: &mut [bool], index: usize) -> usize {
        let mut total = 1;
        for program_id in &self.numbers[index] {
            if !seen[*program_id] {
                seen[*program_id] = true;
                total += self.search_programs(seen, *program_id);
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, 2);
    }
}
