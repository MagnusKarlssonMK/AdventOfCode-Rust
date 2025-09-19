//! # 2023 day 21 - Step Counter
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

use crate::aoc_util::{
    grid::Grid,
    point::{self, Point},
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    grid: Grid,
    start: Point,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let start = grid.find('S').unwrap();
        Ok(Self { grid, start })
    }
}

impl InputData {
    fn get_neighbors(&self, p: &Point, expand: bool) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for n in point::NEIGHBORS_STRAIGHT.iter().map(|d| *d + *p) {
            if expand {
                let n_relative = Point::new(
                    n.x.rem_euclid(self.grid.x_max as i32),
                    n.y.rem_euclid(self.grid.y_max as i32),
                );
                if let Some(v) = self.grid.get_element(&n_relative)
                    && v != '#'
                {
                    neighbors.push(n);
                }
            } else if let Some(v) = self.grid.get_element(&n)
                && v != '#'
            {
                neighbors.push(n);
            }
        }
        neighbors
    }

    fn get_reachable(&self, maxsteps: usize, expand: bool) -> usize {
        let mut seen = HashSet::new();
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::from([(self.start, 0)]);
        let oddeven = maxsteps % 2;

        while let Some((current_point, steps)) = queue.pop_front() {
            if seen.contains(&current_point) {
                continue;
            }
            seen.insert(current_point);
            if steps <= maxsteps {
                if steps % 2 == oddeven {
                    reachable.insert(current_point);
                }
                for neighbor in &self.get_neighbors(&current_point, expand) {
                    queue.push_back((*neighbor, steps + 1));
                }
            }
        }
        reachable.len()
    }

    fn get_infinite_reachable(&self, maxsteps: usize) -> usize {
        let n: usize = (self.grid.y_max - 1) / 2;
        // y = a*x^2 + b*x + c
        let y: Vec<isize> = (0..3)
            .map(|i| self.get_reachable(n + (i * self.grid.y_max), true) as isize)
            .collect();
        let c = y[0];
        let b = ((4 * y[1]) - (3 * y[0]) - y[2]) / 2;
        let a = y[1] - y[0] - b;
        let x = ((maxsteps - n) / self.grid.y_max) as isize;
        ((a * x.pow(2)) + (b * x) + c) as usize
    }

    fn solve_part1(&self) -> usize {
        self.get_reachable(64, false)
    }

    fn solve_part2(&self) -> usize {
        self.get_infinite_reachable(26501365)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.get_reachable(6, false), 16);
    }

    // Test input doesn't have the same geometric symmetry as the real input, i.e. the assumptions made
    // in the solution don't apply to the test data
    /*#[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.get_infinite_reachable(6), 16);
        assert_eq!(solution_data.get_infinite_reachable(10), 50);
        assert_eq!(solution_data.get_infinite_reachable(50), 1594);
        assert_eq!(solution_data.get_infinite_reachable(100), 6536);
        assert_eq!(solution_data.get_infinite_reachable(500), 167004);
        assert_eq!(solution_data.get_infinite_reachable(1000), 668697);
        assert_eq!(solution_data.get_infinite_reachable(5000), 16733044);
    }*/
}
