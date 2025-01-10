//! # 2024 day 6 - Guard Gallivant
//!
//! Solves both parts in one function, since the result from part 1 is
//! used for part 2 to determine possible roadblock positions.
//!
//! Part 2 solution is brute force and crawls through the simulation one
//! step at the time and is thus really slow. Although this method is
//! necessary for part 1 to find all points, for part 2 it would be enough
//! to pre-calculate a graph and directly jump from one roadblock to the
//! next. On todo-list!
use crate::aoc_util::point::*;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    obstacles: HashSet<Point>,
    x_max: i32,
    y_max: i32,
    guard_start_pos: Point,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Point> = None;
        let mut obstacles = HashSet::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in s.lines().enumerate() {
            y_max += 1;
            if y == 0 {
                x_max = line.len() as i32;
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.insert(Point::new(x as i32, y as i32));
                } else if c == '^' {
                    start = Some(Point::new(x as i32, y as i32));
                }
            }
        }
        Ok(Self {
            obstacles,
            x_max,
            y_max,
            guard_start_pos: start.unwrap(),
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut guard = self.guard_start_pos;
        let mut guard_dir = UP;
        while (0..self.x_max).contains(&guard.x) && (0..self.y_max).contains(&guard.y) {
            visited.insert(guard);
            let next_point = guard + guard_dir;
            if self.obstacles.contains(&next_point) {
                guard_dir = guard_dir.rotate_right();
            } else {
                guard = next_point;
            }
        }

        let mut p2 = 0;
        for extra_obstacle in &visited {
            if *extra_obstacle == self.guard_start_pos {
                continue;
            }
            guard = self.guard_start_pos;
            guard_dir = UP;
            let mut visited_directed: HashSet<(Point, Point)> = HashSet::new();
            while (0..self.x_max).contains(&guard.x) && (0..self.y_max).contains(&guard.y) {
                if visited_directed.contains(&(guard, guard_dir)) {
                    p2 += 1;
                    break;
                }
                visited_directed.insert((guard, guard_dir));
                let next_point = guard + guard_dir;
                if self.obstacles.contains(&next_point) || next_point == *extra_obstacle {
                    guard_dir = guard_dir.rotate_right();
                } else {
                    guard = next_point;
                }
            }
        }

        (visited.len(), p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 41);
        assert_eq!(p2, 6);
    }
}
