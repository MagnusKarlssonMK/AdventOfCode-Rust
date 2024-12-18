//! # 2024 day 18 - RAM Run
use crate::aoc_util::{grid::*, point::*};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input, 71);
    println!("Part 1: {}", solution_data.solve_part1(1024));
    println!("Part 2: {}", solution_data.solve_part2(1024));
}

impl Point {
    fn tostring(&self) -> String {
        self.x.to_string() + "," + &self.y.to_string()
    }
}

struct InputData {
    bytes: Vec<Point>,
    max_x: usize,
    max_y: usize,
}

impl InputData {
    fn parse_input(input: &str, dim: usize) -> Self {
        Self {
            bytes: input
                .lines()
                .map(|line| Point::from_str(line).unwrap())
                .collect(),
            max_x: dim,
            max_y: dim,
        }
    }

    fn shortest_path(&self, bytes: usize) -> Option<usize> {
        let mut grid = Grid::new(self.max_x, self.max_y, '.');
        (0..bytes).for_each(|i| grid.set_point(&self.bytes[i], '#'));
        let mut seen = HashSet::new();
        let exit = Point::new(self.max_x as i32 - 1, self.max_y as i32 - 1);
        let mut queue = VecDeque::new();
        queue.push_back((ORIGIN, 0, Point::new(-1, -1)));

        while let Some((current, steps, previous)) = queue.pop_front() {
            if current == exit {
                return Some(steps);
            }
            if seen.contains(&current) {
                continue;
            }
            seen.insert(current);
            for neighbor in NEIGHBORS_STRAIGHT.iter().map(|n| *n + current) {
                if let Some(v) = grid.get_element(&neighbor) {
                    if v == '#' || neighbor == previous {
                        continue;
                    }
                    queue.push_back((neighbor, steps + 1, current));
                }
            }
        }
        None
    }

    fn solve_part1(&self, bytes: usize) -> usize {
        self.shortest_path(bytes).unwrap()
    }

    fn solve_part2(&self, start: usize) -> String {
        let mut lowest = start;
        let mut highest = self.bytes.len();
        while lowest < highest - 1 {
            let tryval = (highest + lowest) / 2;
            if self.shortest_path(tryval).is_some() {
                lowest = tryval;
            } else {
                highest = tryval;
            }
        }
        self.bytes[lowest].tostring()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        );
        let solution_data = InputData::parse_input(&testdata, 7);
        assert_eq!(solution_data.solve_part1(12), 22);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        );
        let solution_data = InputData::parse_input(&testdata, 7);
        assert_eq!(solution_data.solve_part2(12), "6,1");
    }
}
