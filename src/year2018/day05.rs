//! # 2018 day 5 - Alchemical Reduction
//!
//! Reaction the input polymer once for part 1, and then uses the output from that as basis for
//! part 2, since it shaves down the size of the original input quite a bit.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const OFFSET: u8 = b'a' - b'A';

fn react(polymer: &[u8]) -> Vec<u8> {
    let mut poly = Vec::with_capacity(polymer.len());
    for c in polymer {
        if let Some(p) = poly.last()
            && c.abs_diff(*p) == OFFSET
        {
            poly.pop();
        } else {
            poly.push(*c)
        }
    }
    poly
}

struct InputData {
    polymer: Vec<u8>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut polymer: Vec<_> = s.chars().map(|c| c as u8).collect();
        polymer = react(&polymer);
        Ok(Self { polymer })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.polymer.len()
    }

    fn solve_part2(&self) -> usize {
        (b'A'..=b'Z')
            .map(|c| {
                react(
                    &self
                        .polymer
                        .iter()
                        .copied()
                        .filter(|v| *v != c && *v != c + OFFSET)
                        .collect::<Vec<_>>(),
                )
                .len()
            })
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
