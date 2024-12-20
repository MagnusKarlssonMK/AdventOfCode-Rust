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
use crate::aoc_util::{grid::*, point::*};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    path: Vec<Point>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let grid = Grid::parse(input);
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
                    if let Some(previous) = grid_path.get(grid_path.len().wrapping_sub(2)) {
                        if *previous == neighbor {
                            continue;
                        }
                    };
                    grid_path.push(neighbor);
                    break;
                }
            }
        }
        Self { path: grid_path }
    }

    fn find_cheats(&self, cheat_steps: usize, gain: usize) -> usize {
        self.path
            .iter()
            .enumerate()
            .map(|(i, p)| {
                (i + gain..self.path.len())
                    .filter(|&j| {
                        p.manhattan(&self.path[j]) <= cheat_steps
                            && j - i - self.path[i].manhattan(&self.path[j]) >= gain
                    })
                    .count()
            })
            .sum()
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

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "###############
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
###############",
        );
        let solution_data = InputData::parse_input(&testdata);
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
        let testdata = String::from(
            "###############
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
###############",
        );
        let solution_data = InputData::parse_input(&testdata);
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
