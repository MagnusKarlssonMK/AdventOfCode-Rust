//! # 2024 day 20 - Race Condition
//!
//! Parses the grid and stores the non-wall points in a vector in the order according to THE path
//! from S to E (there is only a single possible path without cheats).
//!
//! Then the number of cheats can be found by for each point on the path compare the manhattan
//! distance with the points 100+ indices away in the vector, and if that manhattan distance
//! is <=(2/20) and the gain after compensating with the travel cost for that distance is still
//! 100+, we found a cheat.
//!
//! Although this is already decently fast, since the solution is calculated on data that doesn't
//! change after parsing, using this as an opportunity to check out multi-threading.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    grid::*,
    point::*,
    thread::*,
};
use std::{
    str::FromStr,
    sync::atomic::{AtomicU32, Ordering},
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    path: Vec<Point>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let start = grid.find(&'S').ctx("start point missing")?;
        let end = grid.find(&'E').ctx("end point missing")?;
        let mut grid_path = vec![start];
        while let Some(&v) = grid_path.last()
            && v != end
        {
            let next = NEIGHBORS_STRAIGHT
                .iter()
                .map(|&n| v + n)
                .find(|neighbor| {
                    grid.get_ref(neighbor).is_some_and(|&e| e != '#')
                        && grid_path.iter().rev().nth(1) != Some(neighbor)
                })
                .ctx("no path from start to end")?;
            grid_path.push(next);
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
                    let distance = p.manhattan(&self.path[j]);
                    distance <= cheat_steps && j - i - distance >= gain
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

    const TEST_OFFICIAL_1: &str = "###############
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
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
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
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
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

    #[test]
    fn parser_no_path() {
        let test_input = "#######\n#S.#.E#\n#######";
        let err = test_input.parse::<InputData>();
        assert!(matches!(err, Err(AocError::Invalid(_))));
    }
}
