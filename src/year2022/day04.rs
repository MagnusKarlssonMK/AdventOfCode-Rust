//! # 2022 day 4 - Camp Cleanup
//!
//! Simply store each line as a pair of elf ranges and check how they relate; Part 1 counts
//! assignments where one range fully contains the other, and Part 2 counts assignments where the
//! ranges overlap at all.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Assignment {
    elf1: (u32, u32),
    elf2: (u32, u32),
}

impl FromStr for Assignment {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> Result<(u32, u32), AocError> {
            let (start, end) = s
                .split_once('-')
                .ctx(format!("expected start-end: {s:?}"))?;
            let start = start.parse::<u32>()?;
            let end = end.parse::<u32>()?;
            if start > end {
                return Err(AocError::Invalid(format!(
                    "range start is greater than end: {s:?}"
                )));
            }
            Ok((start, end))
        }

        let (first, second) = s.split_once(',').ctx(format!("expected a-b,c-d: {s:?}"))?;
        Ok(Self {
            elf1: parse_pair(first)?,
            elf2: parse_pair(second)?,
        })
    }
}

impl Assignment {
    fn contains(outer: (u32, u32), inner: (u32, u32)) -> bool {
        outer.0 <= inner.0 && inner.1 <= outer.1
    }

    fn is_fully_contained(&self) -> bool {
        Self::contains(self.elf1, self.elf2) || Self::contains(self.elf2, self.elf1)
    }

    fn overlaps(&self) -> bool {
        self.elf1.0 <= self.elf2.1 && self.elf2.0 <= self.elf1.1
    }
}

struct InputData {
    assignments: Vec<Assignment>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            assignments: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.assignments
            .iter()
            .filter(|a| a.is_fully_contained())
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.assignments.iter().filter(|a| a.overlaps()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "2-4,6-8-9",   // too many fields
            "2-4",         // missing second pair
            "2-4,",        // missing second pair values
            "2-4,6-8,1-2", // extra pair
            "4-2,6-8",     // reversed first range
            "2-4,8-6",     // reversed second range
            "a-b,c-d",     // non-numeric
        ] {
            assert!(input.parse::<InputData>().is_err());
        }
    }
}
