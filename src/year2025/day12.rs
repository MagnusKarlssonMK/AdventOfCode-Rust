//! # 2025 day 12 - Christmas Tree Farm
//!
//! The actual input is setup in a way that either the number of presents will fit even if every
//! present is given a full 3x3 space, or it will not fit even if the presents are packed with every
//! single square filled.
//!
//! So we can make a much simplified solution with this assumption, and ignore the shapes completely,
//! and instead simply check if the number of presents would fit if they were all 3x3 in size.
//! Making a generic solution that could handle all cases would be much more complex.
//!
//! Note that the test input does not follow these assumptions, and can't be used for this solution.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((solution_data.solve_part1().to_string(), "-".to_string()))
}

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    quantities: Vec<u32>,
}

impl FromStr for Region {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let (width, height) = left.split_once('x').unwrap();
        let width = width.parse().unwrap();
        let height = height.parse().unwrap();
        let quantities = right
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Self {
            width,
            height,
            quantities,
        })
    }
}

struct InputData {
    regions: Vec<Region>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, r) = s.rsplit_once("\n\n").unwrap();
        Ok(Self {
            regions: r
                .lines()
                .map(|line| Region::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.regions
            .iter()
            .filter(|region| {
                region.height * region.width >= region.quantities.iter().sum::<u32>() * 3 * 3
            })
            .count()
    }
}
