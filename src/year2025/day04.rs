//! # 2025 day 4 - Printing Department
//!
//! ## Part 1
//!
//! * Build a queue of the points containing paper ('@'), to minimize the number of points that
//!   need to have their neighbors scanned.
//! * Iterate over those points and check all 8 neighbors. If the number of neighbors containing
//!   paper is less than 4, increment the total result.
//!
//! ## Part 2
//!
//! Despite initially looking like a game-of-life problem, it's actually not, since no nodes ever
//! grow back to contain paper. So there is actually no need to iterate in rounds as in the
//! example in the problem description. Instead, we can remove identified points immediately,
//! and then add its neighbors back to the queue for re-evaluation. So,
//!
//! * Start similar to part 1, but also create a copy of the map to keep track of current state
//!   as paper gets removed.
//! * When a point is identified to be cleared, update that point in the map and push all its
//!   neighbors containing paper to the back of the queue to be re-evaluated.
//! * Once the queue is empty, count the number of nodes in the map that still contain paper.
use std::{collections::VecDeque, error::Error, str::FromStr};

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

const PAPER: char = '@';
const NOT_PAPER: char = '.';

impl InputData {
    fn solve_part1(&self) -> u32 {
        let mut total = 0;
        let paper: Vec<_> = self
            .paper_map
            .elements
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == PAPER)
            .map(|(i, _)| self.paper_map.get_point(i))
            .collect();
        let mut queue = VecDeque::from(paper);
        while let Some(p) = queue.pop_front() {
            let mut paper_neighbors = 0;
            for neighbor in NEIGHBORS_ALL.iter().map(|n| *n + p) {
                if let Some(c) = self.paper_map.get_element(&neighbor)
                    && c == PAPER
                {
                    paper_neighbors += 1;
                }
            }
            if paper_neighbors < 4 {
                total += 1;
            }
        }
        total
    }

    fn solve_part2(&self) -> u32 {
        let mut total = 0;
        let mut paper_map = self.paper_map.clone();
        let paper: Vec<_> = self
            .paper_map
            .elements
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == PAPER)
            .map(|(i, _)| self.paper_map.get_point(i))
            .collect();
        let mut queue = VecDeque::from(paper);
        while let Some(p) = queue.pop_front() {
            if paper_map.get_element(&p).unwrap() != PAPER {
                // The point has already been cleared after this entry got added to the queue
                continue;
            }
            let mut paper_neighbors = Vec::with_capacity(8);
            for neighbor in NEIGHBORS_ALL.iter().map(|n| *n + p) {
                if let Some(c) = paper_map.get_element(&neighbor)
                    && c == PAPER
                {
                    paper_neighbors.push(neighbor);
                }
            }
            if paper_neighbors.len() < 4 {
                total += 1;
                paper_map.set_point(&p, NOT_PAPER);
                for np in &paper_neighbors {
                    queue.push_back(*np);
                }
            }
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
