//! # 2023 day 17 - Clumsy Crucible
//!
//! Traverses the map with A* algorithm, using the manhattan distance to the target
//! as heuristic.
use std::{
    collections::{BinaryHeap, HashSet},
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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    point: Point,
    dir: Point,
    steps: u8,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    heuristic: usize,
    heat: usize,
    pos: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heuristic
            .cmp(&self.heuristic)
            .then_with(|| other.heat.cmp(&self.heat))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
    fn get_shortest_path(&self, minsteps: u8, maxsteps: u8) -> usize {
        let start = Point::new(0, 0);
        let target = self.grid.get_point(self.grid.elements.len() - 1);
        let mut queue = BinaryHeap::new();
        let mut seen = HashSet::new();
        // Starting direction can be both Right and Down
        queue.push(State {
            heuristic: start.manhattan(&target),
            heat: 0,
            pos: Position {
                point: start,
                dir: point::RIGHT,
                steps: 0,
            },
        });
        queue.push(State {
            heuristic: start.manhattan(&target),
            heat: 0,
            pos: Position {
                point: start,
                dir: point::DOWN,
                steps: 0,
            },
        });
        while let Some(state) = queue.pop() {
            // Reaching target only a solution if step counter is at least minsteps
            if state.pos.point == target && state.pos.steps >= minsteps {
                return state.heat;
            }
            if seen.contains(&state.pos) {
                continue;
            }
            seen.insert(state.pos);
            let mut nextstates = Vec::new();
            if state.pos.steps >= minsteps {
                let left = state.pos.dir.rotate_left();
                nextstates.push(Position {
                    point: state.pos.point + left,
                    dir: left,
                    steps: 1,
                });
                let right = state.pos.dir.rotate_right();
                nextstates.push(Position {
                    point: state.pos.point + right,
                    dir: right,
                    steps: 1,
                });
            }
            if state.pos.steps < maxsteps {
                nextstates.push(Position {
                    point: state.pos.point + state.pos.dir,
                    dir: state.pos.dir,
                    steps: state.pos.steps + 1,
                });
            }
            while let Some(ns) = nextstates.pop() {
                if let Some(e) = self.grid.get_element(&ns.point) {
                    let new_heat = state.heat + e.to_digit(10).unwrap() as usize;
                    queue.push(State {
                        heuristic: new_heat + ns.point.manhattan(&target),
                        heat: new_heat,
                        pos: ns,
                    });
                }
            }
        }
        0
    }

    fn solve_part1(&self) -> usize {
        self.get_shortest_path(0, 3)
    }

    fn solve_part2(&self) -> usize {
        self.get_shortest_path(4, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 102);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 94);
    }

    #[test]
    fn part2_example_2() {
        let test_data = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let solution_data = InputData::from_str(test_data).unwrap();
        assert_eq!(solution_data.solve_part2(), 71);
    }
}
