//! # 2025 day 11 - Reactor
//!
//! ## Part 1
//!
//! Simple recursive DFS to count number of paths from "you" to "out".
//!
//! ## Part 2
//!
//! Similar solution as in part 1, but now also passing along whether or not "fft" and "dac"
//! have been found in the recursion.
use crate::aoc_util::error::{AocError, OptionExt};
use std::collections::HashMap;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    tree: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut tree = HashMap::new();
        for line in s.lines() {
            let (left, right) = line.split_once(": ").ctx("expected 'key: destinations'")?;
            if left.is_empty() || left.chars().any(char::is_whitespace) {
                return Err(AocError::Invalid(format!("invalid key: {left}")));
            }
            let destinations: Vec<_> = right.split_whitespace().collect();
            if tree.insert(left, destinations).is_some() {
                return Err(AocError::Invalid(format!("duplicate key: {left}")));
            }
        }
        Ok(Self { tree })
    }
}

impl<'a> InputData<'a> {
    fn find_nbr_paths_straight(&self, key: &'a str, seen: &mut HashMap<&'a str, u64>) -> u64 {
        if key == "out" {
            return 1;
        }
        let mut total = 0;
        if let Some(connections) = self.tree.get(key) {
            for &connection in connections {
                if let Some(&v) = seen.get(connection) {
                    total += v;
                } else {
                    let v = self.find_nbr_paths_straight(connection, seen);
                    seen.insert(connection, v);
                    total += v;
                }
            }
        }
        total
    }

    fn find_nbr_paths_fft_dac(
        &self,
        key: &'a str,
        seen: &mut HashMap<(&'a str, bool, bool), u64>,
        fft: bool,
        dac: bool,
    ) -> u64 {
        if key == "out" {
            return u64::from(fft && dac);
        }
        let fft = fft || key == "fft";
        let dac = dac || key == "dac";
        let mut total = 0;
        if let Some(connections) = self.tree.get(key) {
            for &connection in connections {
                let cache_key = (connection, fft, dac);
                if let Some(&v) = seen.get(&cache_key) {
                    total += v;
                } else {
                    let v = self.find_nbr_paths_fft_dac(connection, seen, fft, dac);
                    seen.insert(cache_key, v);
                    total += v;
                }
            }
        }
        total
    }

    fn solve_part1(&self) -> u64 {
        let mut seen = HashMap::new();
        self.find_nbr_paths_straight("you", &mut seen)
    }

    fn solve_part2(&self) -> u64 {
        let mut seen = HashMap::new();
        self.find_nbr_paths_fft_dac("svr", &mut seen, false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_OFFICIAL_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "aaa: bbb\nmissing separator",
            ": bbb",
            "a b: ccc",
            "aaa: bbb\naaa: ccc",
        ] {
            assert!(
                InputData::try_from(input).is_err(),
                "expected error for {input:?}"
            );
        }
    }
}
