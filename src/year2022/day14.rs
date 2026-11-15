//! # 2022 day 14 - Regolith Reservoir
//!
//! ## Parts 1 & 2
//!
//! Simulates the falling sand by recording a trail so we can go back to the previous position and
//! continue from there when a grain either gets locked in place or falls off the map into the
//! abyss. This makes it so that we don't need to start over from the start point for each grain,
//! which greatly reduces the number of iterations required.
//!
//! The answer for both parts can be determined in the same simulation by recording when a grain of
//! sand would first fall off the map (reaching the maximum y value, beyond which we know there are
//! no other rocks), and then proceed to lock it in place, treating it as having hit a blocking
//! rock.
use crate::aoc_util::{error::AocError, point::*};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse().unwrap();
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

const SOURCE_POINT: Point = Point::new(500, 0);

struct InputData {
    rock_joints: Vec<Vec<Point>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rock_joints: s
                .lines()
                .map(|line| {
                    line.split(" -> ")
                        .map(|s| Point::from_str(s))
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut rocks = HashSet::new();
        for joints in &self.rock_joints {
            // Note: any joint with a just a single point in it will effectively be ignored; they
            // don't form any lines at all.
            for (j1, j2) in joints.iter().zip(joints.iter().skip(1)) {
                let delta = Point::new((j2.x - j1.x).signum(), (j2.y - j1.y).signum());
                if delta.x != 0 && delta.y != 0 {
                    // Non-straight lines would likely cause the loop below to hang
                    return Err(AocError::Invalid(format!(
                        "not a straight line: {:?} -> {:?}",
                        j1, j2
                    )));
                }
                let mut p = *j1;
                while p != *j2 {
                    rocks.insert(p);
                    p += delta;
                }
                rocks.insert(p);
            }
        }
        let max_y = rocks
            .iter()
            .map(|p| p.y)
            .max()
            .expect("rocks cannot be empty");
        let start = SOURCE_POINT;
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
                    // Maybe just return NoSolution instead, just to not report a seemingly valid result by accident?
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
        if let Some(p1) = p1 {
            Ok((p1, count))
        } else {
            Err(AocError::NoSolution)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 24);
        assert_eq!(p2, 93);
    }

    #[test]
    fn part1_2_custom_1() {
        // A long horizontal line right under the entrance -> no solution possible for part 1
        let solution_data: InputData = "490,2 -> 510,2".parse().unwrap();
        let err = solution_data.solve();
        assert_eq!(err, Err(AocError::NoSolution));
    }

    #[test]
    fn part1_2_custom_2() {
        // A single non- horizontal/vertical line -> not allowed
        let solution_data: InputData = "490,2 -> 510,3".parse().unwrap();
        let err = solution_data.solve();
        assert!(matches!(err, Err(AocError::Invalid(_))));
    }
}
