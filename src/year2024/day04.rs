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
//! word is eithes MAS or SAM.
use crate::aoc_util::{grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    grid: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::parse(s),
        })
    }
}
impl InputData {
    fn solve_part1(&self) -> usize {
        let mut total = 0;
        for (i, c) in self.grid.elements.iter().enumerate() {
            let current = self.grid.get_point(i);
            if *c == 'X' {
                for dir in &NEIGHBORS_ALL {
                    let word: String = [
                        'X',
                        self.grid.get_element(&(current + *dir)).unwrap_or('.'),
                        self.grid.get_element(&(current + *dir * 2)).unwrap_or('.'),
                        self.grid.get_element(&(current + *dir * 3)).unwrap_or('.'),
                    ]
                    .iter()
                    .collect();
                    if word == "XMAS" {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    fn solve_part2(&self) -> usize {
        let mut total = 0;
        for (i, c) in self.grid.elements.iter().enumerate() {
            let current = self.grid.get_point(i);
            if *c == 'A' {
                let dir = Point::new(1, 1);
                let word1: String = [
                    self.grid.get_element(&(current - dir)).unwrap_or('.'),
                    'A',
                    self.grid.get_element(&(current + dir)).unwrap_or('.'),
                ]
                .iter()
                .collect();
                if word1 == "MAS" || word1 == "SAM" {
                    let dir2 = Point::new(1, -1);
                    let word2: String = [
                        self.grid.get_element(&(current - dir2)).unwrap_or('.'),
                        'A',
                        self.grid.get_element(&(current + dir2)).unwrap_or('.'),
                    ]
                    .iter()
                    .collect();
                    if word2 == "MAS" || word2 == "SAM" {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "MMMSXXMASM
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
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 9);
    }
}
