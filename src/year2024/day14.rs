//! # 2024 day 14 - Restroom Redoubt
//!
//! ## Part 1
//!
//! No need to run the simulation for 100 steps, just directly calculate the positions. Note that
//! the % operator in rust gives remainder, not modulus. I.e. -3 % 10 = -3, which is not what we
//! want, so we need to use the rem_euclid function instead.
//!
//! ## Part 2
//!
//! Run the simulation until all robots are in unique positions with no overlap.
use crate::aoc_util::{error::AocError, point::*};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const X_MAX: i32 = 101;
const Y_MAX: i32 = 103;

struct Robot {
    pos: Point,
    vel: Point,
}

impl FromStr for Robot {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or_else(|| AocError::Invalid(format!("invalid robot: {s}")))?;
        let left = left
            .strip_prefix("p=")
            .ok_or_else(|| AocError::Invalid(format!("invalid robot: {s}")))?;
        let right = right
            .strip_prefix("v=")
            .ok_or_else(|| AocError::Invalid(format!("invalid robot: {s}")))?;
        Ok(Self {
            pos: left.parse()?,
            vel: right.parse()?,
        })
    }
}

struct InputData {
    robots: Vec<Robot>,
    x_max: i32,
    y_max: i32,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            robots: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
            x_max: X_MAX,
            y_max: Y_MAX,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        let middle_x = ((self.x_max - 1) / 2) as usize;
        let middle_y = ((self.y_max - 1) / 2) as usize;
        let mut quadrants: [u64; 4] = [0; 4];
        for r in &self.robots {
            let x_100 = (r.pos.x + 100 * r.vel.x).rem_euclid(self.x_max) as usize;
            let y_100 = (r.pos.y + 100 * r.vel.y).rem_euclid(self.y_max) as usize;
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
                    (r.pos.x + time * r.vel.x).rem_euclid(self.x_max),
                    (r.pos.y + time * r.vel.y).rem_euclid(self.y_max),
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

    const TEST_OFFICIAL_1: &str = "p=0,4 v=3,-3
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

    #[test]
    fn part1_official_1() {
        let mut solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        solution_data.x_max = 11;
        solution_data.y_max = 7;
        assert_eq!(solution_data.solve_part1(), 12);
    }
}
