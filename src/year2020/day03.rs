//! # 2020 day 3 - Toboggan Trajectory
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
    fn count_trees(&self, step: &Point) -> usize {
        let mut pos = ORIGIN;
        let mut count = 0;
        while let Some(e) = self.grid.get_element(&pos) {
            if e == '#' {
                count += 1;
            }
            pos += *step;
            pos.x %= self.grid.x_max as i32;
        }
        count
    }

    fn solve_part1(&self) -> usize {
        self.count_trees(&Point::new(3, 1))
    }

    fn solve_part2(&self) -> usize {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(x, y)| self.count_trees(&Point::new(*x, *y)))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 336);
    }
}
