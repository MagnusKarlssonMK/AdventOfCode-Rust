//! # 2024 day 20 - Race Condition
//!
//! Parses the grid and stores the non-wall points in a vector in the order
//! according to THE path from S to E (there is only a single possible path
//! without cheats).
//!
//! Then the number of cheats can be found by for each point on the path compare
//! the manhattan distance with the points 100+ indices away in the vector,
//! and if that manhattan distance is <=(2/20) and the gain after compensating
//! with the travel cost for that distance is still 100+, we found a cheat.
//!
//! Although this is already decently fast, since the solution is calculated on
//! data that doesn't change after parsing, using this to as an opportunity
//! to check out multi-threading.
use crate::aoc_util::{grid::*, point::*, thread::*};
use std::{
    error::Error,
    str::FromStr,
    sync::atomic::{AtomicU32, Ordering},
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    path: Vec<Point>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let start = grid.find('S').unwrap();
        let end = grid.find('E').unwrap();
        let mut grid_path = vec![start];
        while *grid_path.last().unwrap() != end {
            for neighbor in NEIGHBORS_STRAIGHT
                .iter()
                .map(|&n| *grid_path.last().unwrap() + n)
            {
                if let Some(e) = grid.get_element(&neighbor) {
                    if e == '#' {
                        continue;
                    }
                    if let Some(previous) = grid_path.get(grid_path.len().wrapping_sub(2))
                        && *previous == neighbor
                    {
                        continue;
                    }
                    grid_path.push(neighbor);
                    break;
                }
            }
        }
        Ok(Self { path: grid_path })
    }
}

impl InputData {
    fn find_cheats(&self, cheat_steps: usize, gain: usize) -> usize {
        let total = AtomicU32::new(0);
        let items = Vec::from_iter(0..self.path.len());
        spawn_jobs(items, |i| self.worker(&total, cheat_steps, gain, i));
        total.into_inner() as usize
    }

    fn worker(&self, total: &AtomicU32, cheat_steps: usize, gain: usize, idx: Vec<usize>) {
        let mut cheats = 0;
        for &i in &idx {
            let p = self.path[i];
            cheats += (i + gain..self.path.len())
                .filter(|&j| {
                    p.manhattan(&self.path[j]) <= cheat_steps
                        && j - i - p.manhattan(&self.path[j]) >= gain
                })
                .count() as u32;
        }
        total.fetch_add(cheats, Ordering::Relaxed);
    }

    fn solve_part1(&self) -> usize {
        self.find_cheats(2, 100)
    }

    fn solve_part2(&self) -> usize {
        self.find_cheats(20, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.find_cheats(2, 2), 44);
        assert_eq!(solution_data.find_cheats(2, 4), 30);
        assert_eq!(solution_data.find_cheats(2, 6), 16);
        assert_eq!(solution_data.find_cheats(2, 8), 14);
        assert_eq!(solution_data.find_cheats(2, 10), 10);
        assert_eq!(solution_data.find_cheats(2, 12), 8);
        assert_eq!(solution_data.find_cheats(2, 20), 5);
        assert_eq!(solution_data.find_cheats(2, 36), 4);
        assert_eq!(solution_data.find_cheats(2, 38), 3);
        assert_eq!(solution_data.find_cheats(2, 40), 2);
        assert_eq!(solution_data.find_cheats(2, 64), 1);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.find_cheats(20, 50), 285);
        assert_eq!(solution_data.find_cheats(20, 52), 253);
        assert_eq!(solution_data.find_cheats(20, 54), 222);
        assert_eq!(solution_data.find_cheats(20, 56), 193);
        assert_eq!(solution_data.find_cheats(20, 58), 154);
        assert_eq!(solution_data.find_cheats(20, 60), 129);
        assert_eq!(solution_data.find_cheats(20, 62), 106);
        assert_eq!(solution_data.find_cheats(20, 64), 86);
        assert_eq!(solution_data.find_cheats(20, 66), 67);
        assert_eq!(solution_data.find_cheats(20, 68), 55);
        assert_eq!(solution_data.find_cheats(20, 70), 41);
        assert_eq!(solution_data.find_cheats(20, 72), 29);
        assert_eq!(solution_data.find_cheats(20, 74), 7);
        assert_eq!(solution_data.find_cheats(20, 76), 3);
    }
}
