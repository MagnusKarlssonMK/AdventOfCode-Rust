//! # 2017 day 11 - Hex Ed
//!
//! Stores the hexgrid using axial coordinate system, i.e. based on 3d system (q, r, s) but
//! with the third dimension truncated, since the sum of all three must be zero, meaning
//! that it can be calculated when necessary.
//!
//! Solves both parts in one go; part 1 is the distance from where we ended up at the end,
//! and part 2 is the furthest away we were away during the steps.
use std::{
    error::Error,
    ops::{Add, AddAssign},
    str::FromStr,
};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug, Clone, Copy)]
struct Hexpoint {
    q: isize,
    r: isize,
}

impl FromStr for Hexpoint {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Self { q: 0, r: -1 }),
            "ne" => Ok(Self { q: 1, r: -1 }),
            "se" => Ok(Self { q: 1, r: 0 }),
            "s" => Ok(Self { q: 0, r: 1 }),
            "sw" => Ok(Self { q: -1, r: 1 }),
            "nw" => Ok(Self { q: -1, r: 0 }),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Hexpoint {
    fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }

    fn get_distance(&self, other: &Self) -> usize {
        let s1 = -(self.q + self.r);
        let s2 = -(other.q + other.r);
        (((self.r - other.r).abs() + (self.q - other.q).abs() + (s1 - s2).abs()) / 2) as usize
    }
}

impl Add for Hexpoint {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.q + other.q, self.r + other.r)
    }
}

impl AddAssign for Hexpoint {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.q += other.q;
        self.r += other.r;
    }
}

#[derive(Debug)]
struct InputData {
    moves: Vec<Hexpoint>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            moves: s
                .split(',')
                .map(|d| d.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let start_point = Hexpoint::new(0, 0);
        let mut current_location = Hexpoint::new(0, 0);
        let mut current_distance = 0;
        let mut max_distance = 0;
        for m in &self.moves {
            current_location += *m;
            current_distance = current_location.get_distance(&start_point);
            max_distance = max_distance.max(current_distance);
        }
        (current_distance, max_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str("ne,ne,ne").unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3);
        assert_eq!(p2, 3);
    }

    #[test]
    fn part1_2_official_2() {
        let solution_data = InputData::from_str("ne,ne,sw,sw").unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 0);
        assert_eq!(p2, 2);
    }

    #[test]
    fn part1_2_official_3() {
        let solution_data = InputData::from_str("ne,ne,s,s").unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 2);
        assert_eq!(p2, 2);
    }

    #[test]
    fn part1_2_official_4() {
        let solution_data = InputData::from_str("se,sw,se,sw,sw").unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3);
        assert_eq!(p2, 3);
    }
}
