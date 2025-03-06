//! # 2020 day 17 - Conway Cubes
//!
//! Kind of brute force solution using hashsets. Could perhaps be optimized by making
//! use of the fact that the map will evolve symmetrically in the z- and w-dimensions.
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl AddAssign for ConwayCube {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Hash for ConwayCube {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
        state.write_i32(self.z);
        state.write_i32(self.w);
    }
}

struct InputData {
    startpoints: Vec<ConwayCube>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut startpoints = Vec::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    startpoints.push(ConwayCube {
                        x: x as i32,
                        y: y as i32,
                        z: 0,
                        w: 0,
                    });
                }
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
        let mut cubes: HashSet<ConwayCube> = HashSet::from_iter(self.startpoints.clone());
        for _ in 0..rounds {
            let mut empty = HashSet::new();
            let mut nextgen = HashSet::new();
            for cube in &cubes {
                let mut count = 0;
                for n in neighbors.iter().map(|d| *d + *cube) {
                    if cubes.contains(&n) {
                        count += 1;
                    } else {
                        empty.insert(n);
                    }
                }
                if (2..=3).contains(&count) {
                    nextgen.insert(*cube);
                }
            }
            for cube in &empty {
                let mut count = 0;
                for n in neighbors.iter().map(|d| *d + *cube) {
                    if cubes.contains(&n) {
                        count += 1;
                        if count > 3 {
                            break;
                        }
                    }
                }
                if count == 3 {
                    nextgen.insert(*cube);
                }
            }
            cubes = nextgen;
        }
        cubes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = ".#.
..#
###";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 112);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 848);
    }
}
