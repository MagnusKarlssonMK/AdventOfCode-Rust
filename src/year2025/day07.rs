//! # 2025 day 7 - Laboratories
//!
//! ## Part 1
//!
//! Move vertically from top to bottom in the grid, and for every row, find the new x-positions
//! containing a beam after checking for a split, and store them in a hashset to avoid duplication.
//! While doing this, count how many times a split is performed.
//!
//! ## Part 2
//!
//! Similar to part 1, but instead of storing x-positions in a hashset, store a vector of counters
//! for an entire row, counting how many times a beam has entered that position. The answer is then
//! given by the sum of those counters.
use std::{error::Error, str::FromStr};

use crate::aoc_util::{grid::Grid, point::Point};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

const START: char = 'S';
const SPLITTER: char = '^';

struct InputData {
    manifold: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifold = Grid::parse(s);
        Ok(Self { manifold })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut split_count = 0;
        let mut x_counts: Vec<usize> = (0..self.manifold.x_max)
            .map(|x| {
                if self.manifold.elements[x] == START {
                    1
                } else {
                    0
                }
            })
            .collect();
        for y in 1..self.manifold.y_max as i32 {
            let mut new_x_counts = vec![0; self.manifold.x_max];
            for x in (0..self.manifold.x_max).filter(|i| x_counts[*i] > 0) {
                if let Some(c) = self.manifold.get_element(&Point::new(x as i32, y)) {
                    if c == SPLITTER {
                        split_count += 1;
                        new_x_counts[x - 1] += x_counts[x];
                        new_x_counts[x + 1] += x_counts[x];
                    } else {
                        new_x_counts[x] += x_counts[x];
                    }
                }
            }
            x_counts = new_x_counts;
        }
        (split_count, x_counts.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = ".......S.......
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
    fn parts1_2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 21);
        assert_eq!(p2, 40);
    }
}
