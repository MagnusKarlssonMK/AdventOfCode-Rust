//! # 2024 day 19 - Linen Layout
//!
//! Recursive solution with memo, solving both parts in one go.
use crate::aoc_util::error::{AocError, OptionExt};
use std::collections::HashMap;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data = InputData::try_from(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (p, d) = s.split_once("\n\n").ctx("block separator missing")?;
        Ok(Self {
            patterns: p.split(", ").collect(),
            designs: d.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (usize, u64) {
        let mut seen = HashMap::new();
        let mut possible = 0;
        let total = self.designs.iter().fold(0, |acc, d| {
            let count = self.process_designs(&mut seen, d);
            possible += (count > 0) as usize;
            acc + count
        });
        (possible, total)
    }

    fn process_designs<'b>(&self, seen: &mut HashMap<&'b str, u64>, design: &'b str) -> u64 {
        if design.is_empty() {
            1
        } else if let Some(cache) = seen.get(design) {
            *cache
        } else {
            let count = self
                .patterns
                .iter()
                .filter_map(|p| design.strip_prefix(p))
                .map(|s| self.process_designs(seen, s))
                .sum();
            seen.insert(design, count);
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn parts1_2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, 16);
    }
}
