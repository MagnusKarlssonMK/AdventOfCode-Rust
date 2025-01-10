//! # 2024 day 16 - Reindeer Maze
use crate::aoc_util::{grid::*, point::*};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::error::Error;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
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
        other.cost.cmp(&self.cost).then_with(|| Ordering::Less)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct InputData {
    maze: Grid,
    start: Point,
    exit: Point,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maze = Grid::parse(s);
        let start = maze.find('S').unwrap();
        let exit = maze.find('E').unwrap();
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
                if let Some(e) = self.maze.get_element(&newpos) {
                    if e != '#' && newcost < dist[self.maze.get_index(&newpos)][newdir.dir_to_idx()]
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
            let next = [
                (p - d, d, v - 1),
                (p, d.rotate_left(), v - 1000),
                (p, d.rotate_right(), v - 1000),
            ];
            for (np, nd, nv) in next {
                if dist[self.maze.get_index(&np)][nd.dir_to_idx()] == nv {
                    queue.push_back((np, nd, nv));
                    dist[self.maze.get_index(&np)][nd.dir_to_idx()] = usize::MAX;
                }
            }
        }

        (fastest, best_seats.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_example_1() {
        let testdata = "###############
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
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 7036);
        assert_eq!(p2, 45);
    }

    #[test]
    fn part1_2_example_2() {
        let testdata = "#################
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
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 11048);
        assert_eq!(p2, 64);
    }
}
