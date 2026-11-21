//! # 2024 day 4 - Ceres Search
//!
//! ## Part 1
//!
//! Scan the grid and investigate all nodes containing an 'X', and look in all
//! eight directions and see if XMAS is created.
//!
//! ## Part 2
//!
//! This time, scan for nodes containing an 'A' instead, form words from the
//! combination with the diagonal nodes to form the X, and see if the generated
//! word is either MAS or SAM.
use crate::aoc_util::{error::AocError, grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    grid: Grid<char>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::parse(s),
        })
    }
}
impl InputData {
    /// Walks `word` from `point` in direction `dir`, stopping at the first mismatch.
    fn word_matches(&self, mut point: Point, dir: Point, word: &[char]) -> bool {
        word.iter().all(|&expected| {
            let found = self.grid.get_ref(&point).is_some_and(|&c| c == expected);
            point += dir;
            found
        })
    }

    /// Checks the two ends of an X diagonal through `current` for M/S in either order. The centre
    /// is already known to be 'A' by the caller.
    fn is_mas_diagonal(&self, current: Point, dir: Point) -> bool {
        matches!(
            (
                self.grid.get_element(&(current - dir)),
                self.grid.get_element(&(current + dir)),
            ),
            (Some('M'), Some('S')) | (Some('S'), Some('M'))
        )
    }

    fn solve_part1(&self) -> usize {
        let mut total = 0;
        for (current, _) in self.grid.enumerate_points().filter(|(_, c)| **c == 'X') {
            for dir in NEIGHBORS_ALL {
                if self.word_matches(current + dir, dir, &['M', 'A', 'S']) {
                    total += 1;
                }
            }
        }
        total
    }

    fn solve_part2(&self) -> usize {
        let mut total = 0;
        for (current, _) in self.grid.enumerate_points().filter(|(_, c)| **c == 'A') {
            if self.is_mas_diagonal(current, Point::new(1, 1))
                && self.is_mas_diagonal(current, Point::new(1, -1))
            {
                total += 1;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 9);
    }
}
