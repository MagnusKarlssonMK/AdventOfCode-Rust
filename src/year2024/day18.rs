//! # 2024 day 18 - RAM Run
//!
//! ## Part 1
//!
//! Solved with a simple BFS, generating the grid based on the first 1024 points in the input.
//!
//! ## Part 2
//!
//! Re-using the BFS function from part 1, and using bisection to identify the last timestamp with
//! a possible path. As starting values, for lower limit we know from part 1 that at 1024 there is
//! still at least one open path, and for upper limit the length of the input is the largest it can
//! possibly be.
//!
//! As a possible improvement, one could generate the grid already at parsing, marking each point
//! with its index in the input, to avoid having to generate the grid for every attempted time
//! stamp. However the bisection finds the answer in only a handful of tries, so the time saving
//! would be in fractions of a ms.
use crate::aoc_util::error::AocError;
use crate::aoc_util::{grid::*, point::*};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1(TARGET_BYTES)?.to_string(),
        solution_data.solve_part2(TARGET_BYTES),
    ))
}

const TARGET_DIMENSIONS: usize = 71; // 0..=70
const TARGET_BYTES: usize = 1024;

struct InputData {
    bytes: Vec<Point>,
    dims: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bytes: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
            dims: TARGET_DIMENSIONS,
        })
    }
}

impl InputData {
    fn shortest_path(&self, bytes: usize) -> Option<usize> {
        let mut grid = Grid::new(self.dims, self.dims, &'.');
        (0..bytes).for_each(|i| grid.set_point(&self.bytes[i], '#'));
        let mut seen = HashSet::new();
        let exit = Point::new(self.dims as i32 - 1, self.dims as i32 - 1);
        let mut queue = VecDeque::new();
        queue.push_back((ORIGIN, 0, Point::new(-1, -1)));

        while let Some((current, steps, previous)) = queue.pop_front() {
            if current == exit {
                return Some(steps);
            }
            if seen.contains(&current) {
                continue;
            }
            seen.insert(current);
            for neighbor in NEIGHBORS_STRAIGHT.iter().map(|n| *n + current) {
                if let Some(&v) = grid.get_ref(&neighbor) {
                    if v == '#' || neighbor == previous {
                        continue;
                    }
                    queue.push_back((neighbor, steps + 1, current));
                }
            }
        }
        None
    }

    fn solve_part1(&self, bytes: usize) -> Result<usize, AocError> {
        self.shortest_path(bytes).ok_or(AocError::NoSolution)
    }

    fn solve_part2(&self, start: usize) -> String {
        let mut lowest = start;
        let mut highest = self.bytes.len();
        while lowest + 1 < highest {
            let tryval = (highest + lowest) / 2;
            if self.shortest_path(tryval).is_some() {
                lowest = tryval;
            } else {
                highest = tryval;
            }
        }
        self.bytes[lowest].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_official_1() {
        let mut solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        solution_data.dims = 7;
        assert_eq!(solution_data.solve_part1(12).unwrap(), 22);
    }

    #[test]
    fn part2_official_1() {
        let mut solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        solution_data.dims = 7;
        assert_eq!(solution_data.solve_part2(12), "6,1");
    }
}
