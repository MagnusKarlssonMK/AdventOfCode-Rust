//! # 2024 day 14 - Restroom Redoubt
//!
//! ## Part 1
//!
//! No need to run the simulation for 100 steps, just directly calculate the positions.
//! Note that the % operator in rust gives remainder, not modulus. I.e. -3 % 10 = -3, which
//! is not what we want, so we need to use the rem_euclid function instead.
//!
//! ## Part 2
//!
//! Run the simulation until all robots are in unique positions with no overlap.
use crate::aoc_util::point::*;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::parse_input(input, 101, 103);
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Robot {
    pos: Point,
    vel: Point,
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').unwrap();
        let (left_x, left_y) = left.split_once(',').unwrap();
        let (right_x, right_y) = right.split_once(',').unwrap();
        Ok(Self {
            pos: Point::new(
                left_x.strip_prefix("p=").unwrap().parse().unwrap(),
                left_y.parse().unwrap(),
            ),
            vel: Point::new(
                right_x.strip_prefix("v=").unwrap().parse().unwrap(),
                right_y.parse().unwrap(),
            ),
        })
    }
}

struct InputData {
    robots: Vec<Robot>,
    x_max: usize,
    y_max: usize,
}

impl InputData {
    fn parse_input(input: &str, x_max: usize, y_max: usize) -> Self {
        Self {
            robots: input
                .lines()
                .map(|line| Robot::from_str(line).unwrap())
                .collect(),
            x_max,
            y_max,
        }
    }

    fn solve_part1(&self) -> usize {
        let middle_x = (self.x_max - 1) / 2;
        let middle_y = (self.y_max - 1) / 2;
        let mut quadrants: [usize; 4] = [0, 0, 0, 0];
        for r in &self.robots {
            let x_100 = (r.pos.x + 100 * r.vel.x).rem_euclid(self.x_max as i32) as usize;
            let y_100 = (r.pos.y + 100 * r.vel.y).rem_euclid(self.y_max as i32) as usize;
            if x_100 != middle_x && y_100 != middle_y {
                quadrants[x_100 / (middle_x + 1) + 2 * (y_100 / (middle_y + 1))] += 1;
            }
        }
        quadrants.iter().product()
    }

    fn solve_part2(&self) -> usize {
        let mut points: HashSet<Point> = HashSet::with_capacity(self.robots.len());
        let mut time = 0;
        let mut overlap = true;
        while overlap {
            time += 1;
            overlap = false;
            for r in self.robots.iter() {
                let newpoint = Point::new(
                    (r.pos.x + time * r.vel.x).rem_euclid(self.x_max as i32),
                    (r.pos.y + time * r.vel.y).rem_euclid(self.y_max as i32),
                );
                if points.contains(&newpoint) {
                    overlap = true;
                    break;
                } else {
                    points.insert(newpoint);
                }
            }
            points.clear();
        }
        time as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let solution_data = InputData::parse_input(testdata, 11, 7);
        assert_eq!(solution_data.solve_part1(), 12);
    }
}
