//! # 2017 day 22 - Sporifica Virus
//!
//! ## Part 1
//!
//! Stores the infected nodes in a HashSet and simply plays the game-of-life for the
//! specified number of rounds, adding and removing nodes along the way, counting how
//! many times nodes gets infected.
//!
//! ## Part 2
//!
//! Uses a `NodeState` to apply to each infected node which is now stored in a HashMap
//! instead. Once a node becomes clean, it is removed from the map, thus there is no
//! clean value needed in the enum.
use crate::aoc_util::{error::*, grid::Grid, point::*};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1(PART_1_BURSTS).to_string(),
        solution_data.solve_part2(PART_2_BURSTS).to_string(),
    ))
}

const PART_1_BURSTS: usize = 10_000;
const PART_2_BURSTS: usize = 10_000_000;

/// Possible states for an infected node
enum NodeState {
    Weakened,
    Infected,
    Flagged,
}

struct InputData {
    start: Point,
    infected: Vec<Point>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let start = grid.get_point(grid.elements.len() / 2);
        let infected: Vec<_> = grid
            .elements
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(i, _)| grid.get_point(i))
            .collect();
        Ok(Self { start, infected })
    }
}

impl InputData {
    fn solve_part1(&self, bursts: usize) -> usize {
        let mut infected: HashSet<_> = self.infected.iter().copied().collect();
        let mut p = self.start;
        let mut d = UP;
        let mut count = 0;
        for _ in 0..bursts {
            if infected.contains(&p) {
                d = d.rotate_right();
                infected.remove(&p);
            } else {
                d = d.rotate_left();
                infected.insert(p);
                count += 1;
            }
            p += d;
        }
        count
    }

    fn solve_part2(&self, bursts: usize) -> usize {
        let mut infected: HashMap<_, _> = self
            .infected
            .iter()
            .map(|p| (*p, NodeState::Infected))
            .collect();
        let mut p = self.start;
        let mut d = UP;
        let mut count = 0;
        for _ in 0..bursts {
            match infected.get(&p) {
                Some(NodeState::Weakened) => {
                    infected.insert(p, NodeState::Infected);
                    count += 1;
                }
                Some(NodeState::Infected) => {
                    d = d.rotate_right();
                    infected.insert(p, NodeState::Flagged);
                }
                Some(NodeState::Flagged) => {
                    d = d.rotate_left().rotate_left(); // Reverse direction
                    infected.remove(&p);
                }
                None => {
                    // Clean
                    d = d.rotate_left();
                    infected.insert(p, NodeState::Weakened);
                }
            }
            p += d;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "..#
#..
...";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(PART_1_BURSTS), 5587);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(100), 26);
    }
}
