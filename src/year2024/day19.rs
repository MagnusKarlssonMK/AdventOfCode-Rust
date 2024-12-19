//! # 2024 day 19 - Linen Layout
//!
//! Recursive solution with memo, solving both parts in one go.
use std::collections::HashMap;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        let (p, d) = input.split_once("\n\n").unwrap();
        Self {
            patterns: p.split(", ").collect(),
            designs: d.lines().collect(),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let seen = &mut HashMap::new();
        let a: Vec<usize> = self
            .designs
            .iter()
            .map(|design| self.process_designs(seen, design))
            .collect();
        (a.iter().filter(|n| **n > 0).count(), a.iter().sum())
    }

    fn process_designs<'b>(&self, seen: &mut HashMap<&'b str, usize>, design: &'b str) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(cache) = seen.get(design) {
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
        let testdata = String::from(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, 16);
    }
}
