//! # 2023 day 8 - Haunted Wasteland
//!
//! Node names are three uppercase alphanumeric characters, so they are parsed as base-36
//! integers; this also makes the final character of a name available as `node % 36`. The tree is
//! stored as a hash map from each node to its left and right successors.
//!
//! ## Part 1
//!
//! Follow the repeating L/R instruction sequence from `AAA` until reaching `ZZZ`, counting steps.
//!
//! ## Part 2
//!
//! Every path starting at a node ending in `A` ends at one ending in `Z`. Each such path is
//! traversed once, and the least common multiple of the path lengths gives the first step where
//! all ghosts reach a `Z` node simultaneously.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    math,
};
use std::{collections::HashMap, error::Error, str::FromStr};

/// Fixed nodes used by the two parts, expressed as base-36 values.
const START: u32 = 10 * 36 * 36 + 10 * 36 + 10; // AAA
const TARGET: u32 = 35 * 36 * 36 + 35 * 36 + 35; // ZZZ
const SUFFIX_START: u32 = 10; // A
const SUFFIX_TARGET: u32 = 35; // Z

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(Clone, Copy)]
enum Direction {
    Left = 0,
    Right = 1,
}

impl Direction {
    fn new(c: char) -> Result<Self, AocError> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(AocError::Invalid(format!("unknown direction: {c}"))),
        }
    }
}

/// The node identifiers are converted from the 3-character string in the input into an integer,
/// using 36 as base (0-9 + A-Z = 36 chars), and then the tree is stored in a hashmap.
struct InputData {
    sequence: Vec<Direction>,
    nodetree: HashMap<u32, [u32; 2]>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_node(s: &str) -> Result<u32, AocError> {
            let invalid = || AocError::Invalid(format!("invalid node: {s}"));
            // `from_str_radix` accepts lowercase letters and any number of digits, so the
            // input's exact three-character uppercase format has to be checked separately.
            if s.len() != 3
                || !s
                    .bytes()
                    .all(|b| b.is_ascii_digit() || b.is_ascii_uppercase())
            {
                return Err(invalid());
            }
            // Exactly three ASCII base-36 digits, so this cannot fail.
            u32::from_str_radix(s, 36).map_err(|_| invalid())
        }

        let mut nodetree = HashMap::new();
        let (block1, block2) = s.split_once("\n\n").ctx("block separator")?;
        let sequence = block1
            .chars()
            .map(Direction::new)
            .collect::<Result<Vec<_>, _>>()?;
        if sequence.is_empty() {
            return Err(AocError::Invalid("empty direction sequence".to_string()));
        }
        for line in block2.lines() {
            let (from_side, to_side) = line
                .split_once(" = (")
                .ok_or_else(|| AocError::Invalid(format!("opening bracket missing: {line}")))?;
            let from = parse_node(from_side)?;
            let (left, right) = to_side
                .split_once(", ")
                .ok_or_else(|| AocError::Invalid(format!("comma separator missing: {line}")))?;
            let right = right
                .strip_suffix(')')
                .ok_or_else(|| AocError::Invalid(format!("closing bracket missing: {line}")))?;
            let left = parse_node(left)?;
            let right = parse_node(right)?;
            nodetree.insert(from, [left, right]);
        }
        Ok(Self { sequence, nodetree })
    }
}

/// Least common multiple with checked arithmetic, since `math::lcm` can wrap.
fn checked_lcm(a: usize, b: usize) -> Result<usize, AocError> {
    let divisor = math::gcd(a, b);
    b.checked_div(divisor)
        .and_then(|reduced| a.checked_mul(reduced))
        .ok_or_else(|| AocError::Overflow("least common multiple".to_string()))
}

impl InputData {
    /// Walks the tree from `start` until `is_target` holds, returning the number of steps.
    /// A walk that exhausts the `(node, direction)` state space cannot reach the target.
    fn walk(&self, start: u32, is_target: impl Fn(u32) -> bool) -> Result<usize, AocError> {
        let dir_count = self.sequence.len();
        let max_steps = self.nodetree.len().saturating_mul(dir_count);
        let mut current = start;
        let mut steps = 0;
        while !is_target(current) {
            if steps >= max_steps {
                return Err(AocError::NoSolution);
            }
            current = self.nodetree.get(&current).ok_or(AocError::NoSolution)?
                [self.sequence[steps % dir_count] as usize];
            steps += 1;
        }
        Ok(steps)
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        self.walk(START, |node| node == TARGET)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let start_elements: Vec<_> = self
            .nodetree
            .keys()
            .filter(|k| *k % 36 == SUFFIX_START)
            .copied()
            .collect();
        let mut steps = self.sequence.len();
        for element in &start_elements {
            let stepcount = self.walk(*element, |node| node % 36 == SUFFIX_TARGET)?;
            steps = checked_lcm(steps, stepcount)?;
        }
        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_OFFICIAL_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_OFFICIAL_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 2);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 6);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 6);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "LR\n\nAaA = (BBB, CCC)",
            "RL\n\nAAA = (BBB, CCDD)",
            "RL\n\nAAA = (B?B, CCC)",
            "RL\n\nAA = (BBB, CCC)",
            "RL\n\nAAAA = (BBB, CCC)",
            "\n\nAAA = (BBB, CCC)",
        ] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
    }

    #[test]
    fn parser_accepts_digits_and_uppercase() {
        let input = "L\n\n000 = (ZZZ, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        assert!(input.parse::<InputData>().is_ok());
    }

    #[test]
    fn parser_broken_path() {
        assert!(matches!(
            "LR\n\nAAA = (BBB, CCC)\nDDD = (EEE, ZZZ)"
                .parse::<InputData>()
                .unwrap()
                .solve_part1(),
            Err(AocError::NoSolution)
        ));
    }

    #[test]
    fn solve_cycle_without_target() {
        let solution_data: InputData = "LR\n\nAAA = (BBB, BBB)\nBBB = (AAA, AAA)".parse().unwrap();
        assert!(matches!(
            solution_data.solve_part1(),
            Err(AocError::NoSolution)
        ));
    }
}
