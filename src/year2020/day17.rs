//! # 2020 day 17 - Conway Cubes
//!
//! The input is assumed to contain only '.' (inactive) and '#' (active); any other character is
//! treated as inactive.
//!
//! Kind of brute force solution, could perhaps be optimized by making use of the fact that the map
//! will evolve symmetrically in the z- and w-dimensions.
//!
//! ## Part 1
//!
//! Simulates the 3D Conway cube rules for six cycles.
//!
//! ## Part 2
//!
//! Runs the same simulation in four dimensions.
use crate::aoc_util::error::AocError;
use std::ops::Add;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ConwayCube {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Add for ConwayCube {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

struct InputData {
    startpoints: Vec<ConwayCube>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut startpoints = Vec::new();
        for (y, line) in s.lines().enumerate() {
            for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                startpoints.push(ConwayCube {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                });
            }
        }
        Ok(Self { startpoints })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let neighbors: Vec<ConwayCube> = (0..3 * 3 * 3)
            .filter(|&i| i != 3 * 3 * 3 / 2)
            .map(|i| ConwayCube {
                x: i % 3 - 1,
                y: i / 3 % 3 - 1,
                z: i / 9 - 1,
                w: 0,
            })
            .collect();
        self.evolve(&neighbors, 6)
    }

    fn solve_part2(&self) -> usize {
        let neighbors: Vec<_> = (0..3 * 3 * 3 * 3)
            .filter(|&i| i != 3 * 3 * 3 * 3 / 2)
            .map(|i| ConwayCube {
                x: i % 3 - 1,
                y: i / 3 % 3 - 1,
                z: i / 9 % 3 - 1,
                w: i / 27 - 1,
            })
            .collect();
        self.evolve(&neighbors, 6)
    }

    fn evolve(&self, neighbors: &[ConwayCube], rounds: usize) -> usize {
        let mut cubes: HashSet<_> = self.startpoints.iter().copied().collect();

        for _ in 0..rounds {
            let mut counts: HashMap<ConwayCube, u8> = HashMap::new();
            for cube in &cubes {
                for neighbor in neighbors.iter().map(|offset| *offset + *cube) {
                    *counts.entry(neighbor).or_default() += 1;
                }
            }

            cubes = counts
                .into_iter()
                .filter(|(cube, count)| *count == 3 || (*count == 2 && cubes.contains(cube)))
                .map(|(cube, _)| cube)
                .collect();
        }
        cubes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = ".#.
..#
###";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 112);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 848);
    }
}
