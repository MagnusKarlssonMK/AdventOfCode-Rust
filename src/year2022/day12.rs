//! # 2022 day 12 - Hill Climbing Algorithm
//!
//! Simple BFS solution; from start to end in part 1, and then going backwards from end to start in
//! part 2. The only real complication is the check for which neighbors are possible to move to;
//! for part 1, the candidate neighbor point can only be at most one higher than current point (but
//! much lower is allowed), while when going backwards for part 2 the current point can be at most
//! one higher than the candidate neighbor point.
use crate::aoc_util::{error::AocError, grid::*, point::*};
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    grid: Grid<char>,
    start: Point,
    end: Point,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::parse(s);
        let start = grid
            .find(&'S')
            .ok_or(AocError::Invalid("starting point not found".to_string()))?;
        let end = grid
            .find(&'E')
            .ok_or(AocError::Invalid("end point not found".to_string()))?;
        grid.set_point(&start, 'a');
        grid.set_point(&end, 'z');
        Ok(Self { grid, start, end })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<usize, AocError> {
        self.bfs(false)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        self.bfs(true)
    }

    fn bfs(&self, backwards: bool) -> Result<usize, AocError> {
        let origin = if backwards { self.end } else { self.start };
        let mut seen = vec![false; self.grid.elements.len()];
        seen[self.grid.get_index(&origin)] = true;
        let mut queue = VecDeque::from([(origin, 0)]);
        while let Some((current_point, steps)) = queue.pop_front() {
            let current_value = self
                .grid
                .get_element(&current_point)
                .expect("queued points are always on the grid");
            if (!backwards && current_point == self.end) || (backwards && current_value == 'a') {
                return Ok(steps);
            }
            for neighbor in NEIGHBORS_STRAIGHT.iter().map(|&n| n + current_point) {
                if let Some(n_value) = self.grid.get_element(&neighbor) {
                    let index = self.grid.get_index(&neighbor);
                    if seen[index] {
                        continue;
                    }
                    let current_level = u32::from(current_value);
                    if (!backwards && u32::from(n_value) <= current_level + 1)
                        || (backwards && current_level <= u32::from(n_value) + 1)
                    {
                        seen[index] = true;
                        queue.push_back((neighbor, steps + 1));
                    }
                }
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    /// A plateau of 'z' walls S off in both directions. From 'a' the only legal step is onto 'b'
    /// or lower, and walking uphill out of 'a' gains at most one letter per step, so neither the
    /// forward search nor the reversed one can ever reach the plateau.
    const TEST_CUSTOM_1: &str = "Szz
zzz
zzE";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 31);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 29);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), Err(AocError::NoSolution));
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["abc", "Sbc", "abE"] {
            assert!(input.parse::<InputData>().is_err());
        }
    }
}
