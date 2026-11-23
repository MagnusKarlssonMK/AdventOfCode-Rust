//! # 2025 day 7 - Laboratories
//!
//! Solve both parts simultaneously:
//!
//! * Start with a vector of counters corresponding to each column, mapped to the first row where
//!   every value is zero except the starting point which is one.
//! * Move vertically from top to bottom in the grid, and for every row, create a new counter vector
//!   which for every column either increments the value in the same column with the value from the
//!   previous row if there is no splitter, or the columns to the left and right if it is a splitter.
//!   After the last column in the row, the new counter vector is moved to the main one. While doing
//!   this, also count how many times a split is performed.
//! * After processing the last row, the answer for part 1 is the counter for number of splits performed,
//!   and the answer to part 2 is the sum of the counter vector.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    grid::Grid,
};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

const SPLITTER: char = '^';

struct InputData {
    manifold: Grid<char>,
    start: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifold = Grid::parse(s);
        let lines: Vec<&str> = s.lines().collect();
        let first = lines.first().ctx("empty manifold")?;
        let mut starts = first.bytes().enumerate().filter(|(_, b)| *b == b'S');
        let (start, _) = starts.next().ctx("start missing from first row")?;
        if starts.next().is_some() {
            return Err(AocError::Invalid("multiple starts in first row".into()));
        }
        for (y, line) in lines.iter().enumerate() {
            if y > 0 && line.contains('S') {
                return Err(AocError::Invalid("start outside the first row".into()));
            }
            if line.starts_with(SPLITTER) || line.ends_with(SPLITTER) {
                return Err(AocError::Invalid(format!(
                    "splitter at grid edge on row {y}"
                )));
            }
        }
        Ok(Self { manifold, start })
    }
}

impl InputData {
    fn solve(&self) -> (u64, u64) {
        let mut x_counts = vec![0u64; self.manifold.x_max];
        x_counts[self.start] = 1;
        let mut next_counts = vec![0u64; self.manifold.x_max];
        let mut split_count = 0;

        for y in 1..self.manifold.y_max {
            next_counts.fill(0);
            for x in 0..self.manifold.x_max {
                let count = x_counts[x];
                if count == 0 {
                    continue;
                }
                if self.manifold.elements[y * self.manifold.x_max + x] == SPLITTER {
                    split_count += 1;
                    next_counts[x - 1] += count;
                    next_counts[x + 1] += count;
                } else {
                    next_counts[x] += count;
                }
            }
            std::mem::swap(&mut x_counts, &mut next_counts);
        }
        (split_count, x_counts.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 21);
        assert_eq!(p2, 40);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "...\n...", // no start
            ".S.\n.S.", // two starts
            "...\n.S.", // start outside the first row
            ".S.\n^..", // splitter on the left edge
            ".S.\n..^", // splitter on the right edge
        ] {
            assert!(
                matches!(input.parse::<InputData>(), Err(AocError::Invalid(_))),
                "expected Invalid for {input:?}"
            );
        }
    }
}
