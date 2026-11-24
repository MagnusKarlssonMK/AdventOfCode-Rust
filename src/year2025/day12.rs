//! # 2025 day 12 - Christmas Tree Farm
//!
//! The actual input is setup in a way that either the number of presents will fit even if every
//! present is given a full 3x3 space, or it will not fit even if the presents are packed with
//! every single square filled.
//!
//! So we can make a much simplified solution with this assumption, and ignore the shapes
//! completely, and instead simply check if the number of presents would fit if they were all 3x3
//! in size. Making a generic solution that could handle all cases would be much more complex.
//!
//! Note that the test input does not follow these assumptions, and can't be used for this solution.
use crate::aoc_util::error::{AocError, OptionExt};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((solution_data.solve_part1().to_string(), "-".to_string()))
}

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    quantities: Vec<u32>,
}

impl FromStr for Region {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').ctx("missing region separator")?;
        let (width, height) = left.split_once('x').ctx("invalid region size")?;
        let width = width.parse()?;
        let height = height.parse()?;
        let quantities = right
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
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
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, r) = s.rsplit_once("\n\n").ctx("block separator missing")?;
        Ok(Self {
            regions: r
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.regions
            .iter()
            .filter(|region| {
                let area = u64::from(region.width) * u64::from(region.height);
                let needed = region.quantities.iter().map(|&q| u64::from(q)).sum::<u64>() * 9;
                area >= needed
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CUSTOM_1: &str = "0:\n.\n\n3x3: 1\n2x2: 3\n4x4: 2";

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "0:",             // missing block separator
            "0:\n\n3x3 1",    // missing ':'
            "0:\n\n3: 1",     // missing 'x'
            "0:\n\nabcx3: 1", // bad width
            "0:\n\n3x3: n",   // bad quantity
        ] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
    }
}
