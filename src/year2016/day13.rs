//! # 2016 day 13 - A Maze of Twisty Little Cubicles
//!
//! Simple BFS solution, solving both parts in one sweep.
//!
//! Returns AocError::NoSolution if the part 1 target can't be reached in a finite
//! grid. Has no guard for a theoretical case where the part 1 target is enclosed
//! and unreachable in an otherwise infinite grid.
use crate::aoc_util::{error::*, point::*};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve(&PART_1_TARGET_POINT)?;
    Ok((p1.to_string(), p2.to_string()))
}

const PART_1_TARGET_POINT: Point = Point::new(31, 39);
const PART_2_STEP_LIMIT: u32 = 50;

#[derive(Debug)]
struct InputData {
    favorite_number: u32,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            favorite_number: s.parse()?,
        })
    }
}

impl InputData {
    /// Returns `true` when `p` is on the grid and open space for the favorite number in input.
    fn is_open_space(&self, p: &Point) -> bool {
        if p.x < 0 || p.y < 0 {
            false
        } else {
            let val = (p.x * p.x)
                + (3 * p.x)
                + (2 * p.x * p.y)
                + p.y
                + (p.y * p.y)
                + self.favorite_number as i32;
            val.count_ones().is_multiple_of(2)
        }
    }

    fn solve(self, target_point: &Point) -> Result<(u32, u32), AocError> {
        let mut p1 = None;
        let mut p2 = 0;
        let start = Point::new(1, 1);
        let mut seen = HashSet::from([start]);
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((current_point, steps)) = queue.pop_front() {
            if p1.is_none() && current_point == *target_point {
                p1 = Some(steps);
            }
            if steps <= PART_2_STEP_LIMIT {
                p2 += 1;
            }
            if p1.is_some() && steps > PART_2_STEP_LIMIT {
                break;
            }
            for p in NEIGHBORS_STRAIGHT
                .iter()
                .map(|n| current_point + *n)
                .filter(|p| self.is_open_space(p))
            {
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back((p, steps + 1));
                }
            }
        }
        let p1 = p1.ok_or(AocError::NoSolution)?;
        Ok((p1, p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "10";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve(&Point::new(7, 4)).unwrap();
        assert_eq!(p1, 11);
        assert_eq!(p2, 151);
    }

    #[test]
    fn part1_2_official_1_no_p1_solution() {
        // Official favorite number has no solution for part 1 with the real target point
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let err = solution_data.solve(&PART_1_TARGET_POINT).unwrap_err();
        assert_eq!(err, AocError::NoSolution);
    }

    #[test]
    fn parser_non_integer() {
        let err = InputData::from_str("3r").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
