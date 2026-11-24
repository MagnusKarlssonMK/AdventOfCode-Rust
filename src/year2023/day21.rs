//! # 2023 day 21 - Step Counter
//!
//! Part 1: Stores the garden in a grid and finds reachable tiles within the step count limit using
//! BFS, counting only the ones whose step count has the same parity as the limit.
//! Part 2: Solves it with three-point-formula to determine the coefficients in a quadratic
//! formula, and calculate the answer from that.
use crate::aoc_util::{error::AocError, grid::Grid, point::*};
use std::{collections::VecDeque, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    grid: Grid<char>,
    start: Point,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let start = grid
            .find(&'S')
            .ok_or(AocError::Invalid("start point not found".to_string()))?;
        Ok(Self { grid, start })
    }
}

impl InputData {
    /// Counts the tiles reachable within `steps`, keeping only the ones whose step count has the
    /// same parity as `steps`, since the gardener lands on every other tile.
    fn get_reachable(&self, maxsteps: usize, expand: bool) -> usize {
        // Every visited point lies within `maxsteps` of the start in both axes, so a flat array
        // indexed by that offset avoids hashing Points. Only used with modest limits (the three
        // quadratic-fit samples are at most 327 steps apart), so the array stays small.
        let span = 2 * maxsteps + 1;
        let origin = Point::new(
            self.start.x - maxsteps as i32,
            self.start.y - maxsteps as i32,
        );
        let mut seen = vec![false; span * span];
        let mut reachable = 0;
        let mut queue = VecDeque::from([(self.start, 0)]);
        let oddeven = maxsteps % 2;

        while let Some((current_point, steps)) = queue.pop_front() {
            let idx = (current_point.y - origin.y) as usize * span
                + (current_point.x - origin.x) as usize;
            if seen[idx] {
                continue;
            }
            seen[idx] = true;
            if steps % 2 == oddeven {
                reachable += 1;
            }
            // No point enqueuing neighbours one step past the limit; they would only be skipped.
            if steps == maxsteps {
                continue;
            }
            for d in NEIGHBORS_STRAIGHT {
                let n = current_point + d;
                let walkable = if expand {
                    let relative = Point::new(
                        n.x.rem_euclid(self.grid.x_max as i32),
                        n.y.rem_euclid(self.grid.y_max as i32),
                    );
                    self.grid.get_ref(&relative).is_some_and(|&v| v != '#')
                } else {
                    self.grid.get_ref(&n).is_some_and(|&v| v != '#')
                };
                if walkable {
                    queue.push_back((n, steps + 1));
                }
            }
        }
        reachable
    }

    /// The tile count is a quadratic in the number of times the garden repeats, so fit
    /// y = a*x^2 + b*x + c to the counts at three step counts, one grid height apart.
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

    const TEST_OFFICIAL_1: &str = "...........
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
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
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
