//! # 2024 day 1 - Historian Hysteria
//!
//! ## Part 1
//! Store the two lists in sorted vectors, then simply zip them together
//! and calculate the differences between each pair.
//!
//! ## Part 2
//! Make use of the already sorted lists, and iterate over the left side.
//! Keep track of the index of the last element used on the right side
//! to minimize the amount of looping on the right side.
use std::{cmp::Ordering, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        for line in s.lines() {
            let mut nbrs = line.split_whitespace();
            left.push(nbrs.next().unwrap().parse().unwrap());
            right.push(nbrs.next().unwrap().parse().unwrap());
        }
        left.sort_unstable();
        right.sort_unstable();
        Ok(Self { left, right })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut right_idx = 0;
        self.left
            .iter()
            .map(|left_nbr| {
                let mut score = 0;
                for right_nbr in self.right.iter().skip(right_idx) {
                    match right_nbr.cmp(left_nbr) {
                        Ordering::Greater => break,
                        Ordering::Equal => score += left_nbr,
                        Ordering::Less => right_idx += 1,
                    }
                }
                score
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 11);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 31);
    }
}
