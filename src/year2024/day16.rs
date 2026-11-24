//! # 2024 day 16 - Reindeer Maze
//!
//! ## Part 1
//!
//! Dijkstra over states of (position, direction): moving forward costs 1, turning 90 degrees
//! costs 1000. Keep the cheapest cost per state, then take the cheapest cost at the exit.
//!
//! ## Part 2
//!
//! Backtrack from each exit state that reached that cheapest cost. A predecessor is valid when
//! its stored distance matches the reverse step (cost - 1 for a move, cost - 1000 for a turn).
//! The union of positions visited across all such paths is the set of good seats.
use crate::aoc_util::error::{AocError, OptionExt};
use crate::aoc_util::{grid::*, point::*};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

impl Point {
    fn dir_to_idx(&self) -> usize {
        match *self {
            RIGHT => 0,
            DOWN => 1,
            LEFT => 2,
            UP => 3,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
    direction: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct InputData {
    maze: Grid<char>,
    start: Point,
    exit: Point,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maze = Grid::parse(s);
        let start = maze.find(&'S').ctx("start point not found")?;
        let exit = maze.find(&'E').ctx("exit point not found")?;
        Ok(Self { maze, start, exit })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut dist: Vec<_> = (0..self.maze.elements.len())
            .map(|_| [usize::MAX; 4])
            .collect();
        let mut queue = BinaryHeap::new();

        let mut fastest = usize::MAX;

        dist[self.maze.get_index(&self.start)][RIGHT.dir_to_idx()] = 0;
        queue.push(State {
            cost: 0,
            position: self.start,
            direction: RIGHT,
        });

        while let Some(State {
            cost,
            position,
            direction,
        }) = queue.pop()
        {
            if position == self.exit {
                fastest = fastest.min(cost);
                continue;
            }
            let pos_idx = self.maze.get_index(&position);
            if cost > dist[pos_idx][direction.dir_to_idx()] {
                continue;
            }

            for (newcost, newpos, newdir) in [
                (cost + 1, position + direction, direction),
                (cost + 1000, position, direction.rotate_left()),
                (cost + 1000, position, direction.rotate_right()),
            ] {
                if let Some(e) = self.maze.get_element(&newpos)
                    && e != '#'
                    && newcost < dist[self.maze.get_index(&newpos)][newdir.dir_to_idx()]
                {
                    queue.push(State {
                        cost: newcost,
                        position: newpos,
                        direction: newdir,
                    });
                    dist[self.maze.get_index(&newpos)][newdir.dir_to_idx()] = newcost;
                }
            }
        }

        let mut best_seats: HashSet<Point> = HashSet::new();
        let mut queue = VecDeque::new();
        for d in NEIGHBORS_STRAIGHT.iter() {
            if dist[self.maze.get_index(&self.exit)][d.dir_to_idx()] == fastest {
                queue.push_back((self.exit, *d, fastest));
            }
        }
        while let Some((p, d, v)) = queue.pop_front() {
            best_seats.insert(p);
            if p == self.start {
                continue;
            }
            let turn_cost = v.checked_sub(1000);
            let next = [
                Some((p - d, d, v - 1)),
                turn_cost.map(|v| (p, d.rotate_left(), v)),
                turn_cost.map(|v| (p, d.rotate_right(), v)),
            ];
            for (np, nd, nv) in next.into_iter().flatten() {
                if self.maze.get_ref(&np).is_none() {
                    continue;
                }
                let idx = self.maze.get_index(&np);
                let dir = nd.dir_to_idx();
                if dist[idx][dir] == nv {
                    queue.push_back((np, nd, nv));
                    dist[idx][dir] = usize::MAX;
                }
            }
        }

        (fastest, best_seats.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_OFFICIAL_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 7036);
        assert_eq!(p2, 45);
    }

    #[test]
    fn parts1_2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 11048);
        assert_eq!(p2, 64);
    }
}
