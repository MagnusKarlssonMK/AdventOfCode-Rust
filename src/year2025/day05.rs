//! # 2025 day 5 - Cafeteria
//!
//! ## Part 1
//!
//! Trivial - for every available ID, iterate over the ID ranges and if any range contains the
//! available ID, increment the total and skip to the next available ID.
//!
//! ## Part 2
//!
//! Sort the fresh-ID ranges by their lower bound, then merge them in a single pass: each range
//! either extends the previous merged range or starts a new one. Merging touching ranges as well
//! as overlapping ones is harmless for the total, since the sizes simply add up.
//!
//! The ranges are inclusive, so the number of IDs in a range is `1 + upper - lower`.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    fresh_id_ranges: Vec<(u64, u64)>,
    available_ids: Vec<u64>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> Result<(u64, u64), AocError> {
            let (lo, hi) = s
                .split_once('-')
                .ok_or_else(|| AocError::Invalid(format!("invalid range: {s}")))?;
            let lo: u64 = lo.parse()?;
            let hi: u64 = hi.parse()?;
            if lo > hi {
                return Err(AocError::Invalid(format!("reversed range: {s}")));
            }
            Ok((lo, hi))
        }
        let (r, a) = s.split_once("\n\n").ctx("block separator missing")?;
        let ranges: Vec<(u64, u64)> = r.lines().map(parse_range).collect::<Result<_, _>>()?;
        Ok(Self {
            fresh_id_ranges: ranges,
            available_ids: a
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.available_ids
            .iter()
            .filter(|a| {
                self.fresh_id_ranges
                    .iter()
                    .any(|(lo, hi)| (lo..=hi).contains(a))
            })
            .count() as u64
    }

    fn solve_part2(&self) -> u64 {
        let mut ranges = self.fresh_id_ranges.clone();
        ranges.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
        for (lo, hi) in ranges {
            if let Some(last) = merged.last_mut()
                && lo <= last.1.saturating_add(1)
            {
                last.1 = last.1.max(hi);
            } else {
                merged.push((lo, hi));
            }
        }
        merged.iter().map(|(lo, hi)| 1 + hi - lo).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 14);
    }

    #[test]
    fn part2_adjacent_ranges() {
        let solution_data: InputData = "1-5\n6-10\n20-20\n\n0".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 11);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["1-5 6-10\n\n1", "1-x\n\n1", "1-5\n\nx", "1-5"] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
        assert!(matches!(
            "1-5\n10-3\n\n1".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }
}
