//! # 2017 day 12 - Digital Plumber
//!
//! Programs form an undirected graph through their pipes. The answer to part 1 is the size of the
//! connected component containing program 0, and part 2 counts all connected components.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

/// Contains the numbers on the righthand-side of `<->` in the input. The lefthand-side can be
/// derived implicitly by the vector index.
struct InputData {
    numbers: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .skip(2)
                        .map(|n| n.trim_end_matches(',').parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<_, _>>()?,
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

    const TEST_OFFICIAL_1: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, 2);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = "0 <-> 0".parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1);
        assert_eq!(p2, 1);
    }
}
