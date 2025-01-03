//! # 2022 day 14 - Regolith Reservoir
//!
//! ## Parts 1 & 2
//!
//! Simulates the falling sand by recording a trail so we can go back to
//! the previous position and continue from there when a grain either gets
//! locked in place or falls off the map into the abyss. This makes it so
//! that we don't need to start over from the start point for each grain,
//! which greatly reduces the number of iterations required.
use crate::aoc_util::point::*;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    rock_joints: Vec<Vec<Point>>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            rock_joints: input
                .lines()
                .map(|line| {
                    line.split(" -> ")
                        .map(|s| Point::from_str(s).unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut rocks = HashSet::new();
        for joints in &self.rock_joints {
            for (j1, j2) in joints.iter().zip(joints.iter().skip(1)) {
                let delta = Point::new((j2.x - j1.x).signum(), (j2.y - j1.y).signum());
                let mut p = *j1;
                while p != *j2 {
                    rocks.insert(p);
                    p += delta;
                }
                rocks.insert(p);
            }
        }
        let max_y = rocks.iter().map(|p| p.y).max().unwrap();
        let start = Point::new(500, 0);
        let mut current = start;
        let mut p1 = None;
        let mut count = 0;
        let mut trail = VecDeque::new();

        loop {
            if rocks.contains(&current) {
                if let Some(p) = trail.pop_back() {
                    current = p;
                } else {
                    // Shouldn't happen, breaking just in case
                    break;
                }
            }
            if current.y > max_y && p1.is_none() {
                p1 = Some(count);
            }
            if !rocks.contains(&(current + DOWN)) && current.y < max_y + 1 {
                trail.push_back(current);
                current += DOWN;
            } else if !rocks.contains(&(current + DOWN + LEFT)) && current.y < max_y + 1 {
                trail.push_back(current);
                current += DOWN + LEFT;
            } else if !rocks.contains(&(current + DOWN + RIGHT)) && current.y < max_y + 1 {
                trail.push_back(current);
                current += DOWN + RIGHT;
            } else {
                count += 1;
                rocks.insert(current);
            }
            if current == start {
                break;
            }
        }
        (p1.unwrap(), count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 24);
        assert_eq!(p2, 93);
    }
}
