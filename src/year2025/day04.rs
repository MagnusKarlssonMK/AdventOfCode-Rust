//! # 2025 day 04 - Printing Department
//!
//! Kind of brute-force for part 2, can probably be optimized by storing paper neighbors
//! for every removed paper roll each round, and only check those points next iteration,
//! instead of scanning the entire map every time.
use std::{error::Error, str::FromStr};

use crate::aoc_util::{grid::Grid, point::*};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    paper_map: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            paper_map: Grid::parse(s),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut count = 0;
        for (i, c) in self.paper_map.elements.iter().enumerate() {
            if *c == '@' {
                let p = self.paper_map.get_point(i);
                let mut neighbor_count = 0;
                for neighbor in NEIGHBORS_ALL.iter().map(|n| *n + p) {
                    if let Some(nc) = self.paper_map.get_element(&neighbor)
                        && nc == '@'
                    {
                        neighbor_count += 1;
                        if neighbor_count >= 4 {
                            break;
                        }
                    }
                }
                if neighbor_count < 4 {
                    count += 1;
                }
            }
        }
        count
    }

    fn solve_part2(&self) -> usize {
        let mut count = self.paper_map.elements.len();
        let mut total = 0;
        let mut current_map = self.paper_map.clone();
        while count > 0 {
            count = 0;
            let mut next_map = current_map.clone();
            for (i, c) in current_map.elements.iter().enumerate() {
                if *c == '@' {
                    let p = current_map.get_point(i);
                    let mut neighbor_count = 0;
                    for neighbor in NEIGHBORS_ALL.iter().map(|n| *n + p) {
                        if let Some(nc) = current_map.get_element(&neighbor)
                            && nc == '@'
                        {
                            neighbor_count += 1;
                            if neighbor_count >= 4 {
                                break;
                            }
                        }
                    }
                    if neighbor_count < 4 {
                        count += 1;
                        next_map.set_point(&p, '.');
                    }
                }
            }
            total += count;
            current_map = next_map;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 43);
    }
}
