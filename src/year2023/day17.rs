//! # 2023 day 17 - Clumsy Crucible
//!
//! Traverses the map with A* algorithm, using the manhattan distance to the target
//! as heuristic.
use std::{collections::BinaryHeap, error::Error, str::FromStr};

use crate::aoc_util::{
    error::AocError,
    grid::Grid,
    point::{Point, NEIGHBORS_STRAIGHT},
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Position {
    point: Point,
    /// Index into `NEIGHBORS_STRAIGHT`, ordered so a left turn is `dir + 3` and a right turn is `dir + 1`.
    dir: u8,
    steps: u8,
}

#[derive(Clone, Copy, Eq, PartialEq)]
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
    grid: Grid<u8>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::parse(s).try_map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or(AocError::Invalid(c.to_string()))
            })?,
        })
    }
}

impl InputData {
    /// Pushes a successor onto the queue, skipping points outside the grid.
    #[inline]
    fn push_successor(
        &self,
        queue: &mut BinaryHeap<State>,
        target: &Point,
        heat: usize,
        pos: Position,
    ) {
        let Some(e) = self.grid.get_ref(&pos.point) else {
            return;
        };
        let new_heat = heat + *e as usize;
        queue.push(State {
            heuristic: new_heat + pos.point.manhattan(target),
            heat: new_heat,
            pos,
        });
    }

    fn get_shortest_path(&self, minsteps: u8, maxsteps: u8) -> usize {
        let start = Point::new(0, 0);
        let target = self.grid.get_point(self.grid.elements.len() - 1);
        let mut queue = BinaryHeap::new();
        // Flat visited set indexed by (cell, direction, steps) rather than hashing a struct.
        let stride = maxsteps as usize + 1;
        let mut seen = vec![false; self.grid.elements.len() * NEIGHBORS_STRAIGHT.len() * stride];
        // Starting direction can be both Right and Down
        for dir in 0..2 {
            queue.push(State {
                heuristic: start.manhattan(&target),
                heat: 0,
                pos: Position {
                    point: start,
                    dir,
                    steps: 0,
                },
            });
        }
        while let Some(state) = queue.pop() {
            // Reaching target only a solution if step counter is at least minsteps
            if state.pos.point == target && state.pos.steps >= minsteps {
                return state.heat;
            }
            let cell = self.grid.get_index(&state.pos.point);
            let idx =
                (cell * NEIGHBORS_STRAIGHT.len() + state.pos.dir as usize) * stride + state.pos.steps as usize;
            if seen[idx] {
                continue;
            }
            seen[idx] = true;

            let dir = state.pos.dir;
            let point = state.pos.point;
            if state.pos.steps >= minsteps {
                let left = (dir + 3) % NEIGHBORS_STRAIGHT.len() as u8;
                self.push_successor(
                    &mut queue,
                    &target,
                    state.heat,
                    Position {
                        point: point + NEIGHBORS_STRAIGHT[left as usize],
                        dir: left,
                        steps: 1,
                    },
                );
                let right = (dir + 1) % NEIGHBORS_STRAIGHT.len() as u8;
                self.push_successor(
                    &mut queue,
                    &target,
                    state.heat,
                    Position {
                        point: point + NEIGHBORS_STRAIGHT[right as usize],
                        dir: right,
                        steps: 1,
                    },
                );
            }
            if state.pos.steps < maxsteps {
                self.push_successor(
                    &mut queue,
                    &target,
                    state.heat,
                    Position {
                        point: point + NEIGHBORS_STRAIGHT[dir as usize],
                        dir,
                        steps: state.pos.steps + 1,
                    },
                );
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

    const TEST_OFFICIAL_1: &str = "2413432311323
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

    const TEST_OFFICIAL_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 102);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 94);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 71);
    }
}
