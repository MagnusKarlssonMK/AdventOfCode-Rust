//! # 2024 day 10 - Hoof It
//!
//! Solves both parts in one go by using BFS to find trails, but with the
//! simplification that there is no need to keep track of previously visited
//! nodes. Since the only valid neighbors have a value of +1, all paths
//! generated during the traversal are guaranteed to be unique. Thus the answer
//! to part 2 is given by the number of times the number 9 is found, and the
//! answer to part 1 by throwing all the points found for each trailhead into
//! a hashset and then calculating its length.
use crate::aoc_util::point::*;
use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct Grid {
    x_max: usize,
    y_max: usize,
    elements: Vec<u8>,
}

impl Grid {
    #[inline]
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect();
        let x_max = lines[0].len();
        let y_max = lines.len();
        let mut elements = Vec::with_capacity(x_max * y_max);
        lines
            .iter()
            .for_each(|line| line.iter().for_each(|c| elements.push(*c)));
        Self {
            x_max,
            y_max,
            elements,
        }
    }

    #[inline]
    fn get_element(&self, p: &Point) -> Option<u8> {
        if (0..self.x_max).contains(&(p.x as usize)) && (0..self.y_max).contains(&(p.y as usize)) {
            Some(self.elements[self.x_max * (p.y as usize) + (p.x as usize)])
        } else {
            None
        }
    }
}

struct InputData {
    map: Grid,
    trailheads: Vec<Point>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let map = Grid::parse(input);
        let trailheads = map
            .elements
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(|(i, _)| Point::new((i % map.x_max) as i32, (i / map.y_max) as i32))
            .collect();
        Self { map, trailheads }
    }

    fn solve(&self) -> (usize, usize) {
        let mut totalscore = 0;
        let mut totalrating = 0;
        for head in &self.trailheads {
            let mut peaks = HashSet::new();
            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(*head);
            while let Some(current) = queue.pop_front() {
                if self.map.get_element(&current).unwrap() == 9 {
                    peaks.insert(current);
                    totalrating += 1;
                } else {
                    for neighbor in NEIGHBORS_STRAIGHT.map(|dir| dir + current) {
                        if let Some(neighbor_val) = self.map.get_element(&neighbor) {
                            if neighbor_val == self.map.get_element(&current).unwrap() + 1 {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }

            totalscore += peaks.len();
        }
        (totalscore, totalrating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts1_2_example_1() {
        let testdata = String::from(
            "0123
1234
8765
9876",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1);
    }

    #[test]
    fn parts1_2_example_2() {
        let testdata = String::from(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 36);
        assert_eq!(p2, 81);
    }
}
