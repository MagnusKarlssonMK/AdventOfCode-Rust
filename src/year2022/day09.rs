//! # 2022 day 9 - Rope Bridge
//!
//! The rope is a vector of knot positions with the head at index 0. Each motion step advances the
//! head, then every following knot catches up: whenever it is more than one step from its leader
//! in either axis, it moves one step toward it. The tail positions after every step are collected
//! in a set, and the two parts differ only in the number of knots (2 and 10).
use crate::aoc_util::{
    error::{AocError, OptionExt},
    point::*,
};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

impl Point {
    #[inline]
    fn parse_dir(input: &str) -> Result<Self, AocError> {
        match input {
            "R" => Ok(RIGHT),
            "L" => Ok(LEFT),
            "U" => Ok(UP),
            "D" => Ok(DOWN),
            _ => Err(AocError::Invalid(format!("invalid direction: {input}"))),
        }
    }

    /// Moves `self` one step in each axis in the direction of `delta`.
    fn catch_up(&self, delta: Point) -> Self {
        Self {
            x: self.x + delta.x.signum(),
            y: self.y + delta.y.signum(),
        }
    }
}

struct InputData {
    motions: Vec<(Point, u32)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_motion(s: &str) -> Result<(Point, u32), AocError> {
            let (dir, len) = s.split_once(' ').ctx(format!("invalid motion: {s}"))?;
            Ok((Point::parse_dir(dir)?, len.parse()?))
        }
        Ok(Self {
            motions: s.lines().map(parse_motion).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    /// Calculates the number of positions the tail visits at least once for the number of knots
    /// used. `nbr_knots` must be > 0.
    fn get_nbr_tail_positions(&self, nbr_knots: usize) -> usize {
        debug_assert!(nbr_knots > 0);
        let mut knotpos = vec![ORIGIN; nbr_knots];
        let mut tail_seen = HashSet::from([ORIGIN]);
        for &(dir, steps) in &self.motions {
            for _ in 0..steps {
                knotpos[0] += dir;
                for i in 1..nbr_knots {
                    let diff = knotpos[i - 1] - knotpos[i];
                    if diff.x.abs() > 1 || diff.y.abs() > 1 {
                        knotpos[i] = knotpos[i].catch_up(diff);
                    }
                }
                tail_seen.insert(knotpos[nbr_knots - 1]);
            }
        }
        tail_seen.len()
    }

    fn solve_part1(&self) -> usize {
        self.get_nbr_tail_positions(2)
    }

    fn solve_part2(&self) -> usize {
        self.get_nbr_tail_positions(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const TEST_OFFICIAL_2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    /// The head never moves, so the rope stays where it started.
    const TEST_CUSTOM_1: &str = "R 0\nU 0";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 36);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn parser_invalid_input() {
        assert!("X 1".parse::<InputData>().is_err());
        assert!("R x".parse::<InputData>().is_err());
    }
}
