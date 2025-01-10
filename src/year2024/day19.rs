//! # 2024 day 19 - Linen Layout
//!
//! Recursive solution with memo, solving both parts in one go.
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (p, d) = s.split_once("\n\n").unwrap();
        Ok(Self {
            patterns: p.split(", ").collect(),
            designs: d.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (usize, usize) {
        let seen = &mut HashMap::new();
        let result: Vec<usize> = self
            .designs
            .iter()
            .map(|design| self.process_designs(seen, design))
            .collect();
        (
            result.iter().filter(|n| **n > 0).count(),
            result.iter().sum(),
        )
    }

    fn process_designs<'b>(&self, seen: &mut HashMap<&'b str, usize>, design: &'b str) -> usize {
        if design.is_empty() {
            1
        } else if let Some(cache) = seen.get(design) {
            *cache
        } else {
            let count: usize = self
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

    #[test]
    fn part1_example_1() {
        let testdata = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, 16);
    }
}
