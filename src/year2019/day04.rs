//! # 2019 day 4 - Secure Container
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    range_lower: usize,
    range_upper: usize,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.split_once('-').unwrap();
        Ok(Self {
            range_lower: low.parse().unwrap(),
            range_upper: high.parse().unwrap(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        (self.range_lower..=self.range_upper)
            .filter(|n| Self::is_valid(*n, false))
            .count()
    }

    fn solve_part2(&self) -> usize {
        (self.range_lower..=self.range_upper)
            .filter(|n| Self::is_valid(*n, true))
            .count()
    }

    /// A valid password has six digits that never decrease and at least one
    /// pair of adjacent equal digits. When `exactly_two` is set, the pair has
    /// to be a run of exactly two (not part of a longer run).
    fn is_valid(number: usize, exactly_two: bool) -> bool {
        let digits = [
            number / 100_000 % 10,
            number / 10_000 % 10,
            number / 1_000 % 10,
            number / 100 % 10,
            number / 10 % 10,
            number % 10,
        ];
        if digits.windows(2).any(|pair| pair[0] > pair[1]) {
            return false;
        }

        // Lengths of the runs of equal adjacent digits.
        let mut runs = Vec::new();
        let mut start = 0;
        while start < digits.len() {
            let mut end = start;
            while end + 1 < digits.len() && digits[end + 1] == digits[start] {
                end += 1;
            }
            runs.push(end - start + 1);
            start = end + 1;
        }

        if exactly_two {
            runs.contains(&2)
        } else {
            runs.iter().any(|run| *run >= 2)
        }
    }
}
