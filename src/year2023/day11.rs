//! # 2023 day 11 - Cosmic Expansion
//!
//! Stores the coordinates of the galaxies in the input, together with a vec for each axis marking
//! the rows / columns that hold at least one galaxy. Part 1 and part 2 differ only in how wide the
//! empty rows and columns become, so rather than counting the empty lines between each pair of
//! galaxies, the galaxies are moved to the coordinates they would have if the empty rows and
//! columns had already been widened, and the answer is then the sum of the manhattan distances
//! between all pairs.
use crate::aoc_util::{error::AocError, grid::Grid, point::*};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const P1_EXP_RATE: u64 = 2;
const P2_EXP_RATE: u64 = 1_000_000;

struct InputData {
    galaxies: Vec<Point>,
    x_occupied: Vec<bool>,
    y_occupied: Vec<bool>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let galaxies: Vec<_> = grid
            .enumerate_points()
            .filter(|&(_, c)| *c == '#')
            .map(|(p, _)| p)
            .collect();
        let mut x_occupied = vec![false; grid.x_max];
        let mut y_occupied = vec![false; grid.y_max];
        for g in &galaxies {
            x_occupied[g.x as usize] = true;
            y_occupied[g.y as usize] = true;
        }
        Ok(Self {
            galaxies,
            x_occupied,
            y_occupied,
        })
    }
}

fn expanded_positions(occupied: &[bool], exp_rate: u64) -> Vec<u64> {
    let mut positions = vec![0; occupied.len()];
    let mut extra = 0;
    for (i, v) in occupied.iter().enumerate() {
        if !*v {
            extra += exp_rate - 1;
        }
        positions[i] = i as u64 + extra;
    }
    positions
}

impl InputData {
    fn distance_sum(&self, exp_rate: u64) -> u64 {
        let xs = expanded_positions(&self.x_occupied, exp_rate);
        let ys = expanded_positions(&self.y_occupied, exp_rate);
        let mut total = 0;
        let galaxies: Vec<_> = self
            .galaxies
            .iter()
            .map(|g| (xs[g.x as usize], ys[g.y as usize]))
            .collect();
        for (i, g1) in galaxies.iter().enumerate() {
            for g2 in galaxies.iter().skip(i + 1) {
                total += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            }
        }
        total
    }

    fn solve_part1(&self) -> u64 {
        self.distance_sum(P1_EXP_RATE)
    }

    fn solve_part2(&self) -> u64 {
        self.distance_sum(P2_EXP_RATE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 374);
        assert_eq!(solution_data.distance_sum(10), 1030);
        assert_eq!(solution_data.distance_sum(100), 8410);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        // Note: answer not given by official description
        assert_eq!(solution_data.solve_part2(), 82000210);
    }
}
