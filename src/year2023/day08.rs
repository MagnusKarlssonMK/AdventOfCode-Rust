//! # 2023 day 8 - Haunted Wasteland
use crate::aoc_util::math;
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(PartialEq)]
enum Direction {
    Left = 0,
    Right = 1,
}

impl Direction {
    fn new(c: &char) -> Self {
        match *c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

/// The node identifiers are converted from the 3-character string in the input into an integer,
/// using 36 as base (0-9 + A-Z = 36 chars), and then the tree is stored in a hashmap.
struct InputData {
    sequence: Vec<Direction>,
    nodetree: HashMap<u32, (u32, u32)>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodetree = HashMap::new();
        let (block1, block2) = s.split_once("\n\n").unwrap();
        let sequence = block1.chars().map(|c| Direction::new(&c)).collect();
        for line in block2.lines() {
            let (from_side, to_side) = line.split_once(" = (").unwrap();
            let from = u32::from_str_radix(from_side, 36).unwrap();
            let (left, right) = to_side.split_once(", ").unwrap();
            let right = right.strip_suffix(')').unwrap();
            let left = u32::from_str_radix(left, 36).unwrap();
            let right = u32::from_str_radix(right, 36).unwrap();
            nodetree.insert(from, (left, right));
        }
        Ok(Self { sequence, nodetree })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut current_element = u32::from_str_radix("AAA", 36).unwrap();
        let target_element = u32::from_str_radix("ZZZ", 36).unwrap();
        let mut stepcount = 0;
        while current_element != target_element {
            current_element = if self.sequence[stepcount % self.sequence.len()] == Direction::Left {
                self.nodetree.get(&current_element).unwrap().0
            } else {
                self.nodetree.get(&current_element).unwrap().1
            };
            stepcount += 1;
        }
        stepcount
    }

    fn solve_part2(&self) -> usize {
        let start = u32::from_str_radix("A", 36).unwrap();
        let target = u32::from_str_radix("Z", 36).unwrap();
        let start_elements: Vec<_> = self
            .nodetree
            .keys()
            .filter(|k| *k % 36 == start)
            .copied()
            .collect();
        let mut steps = self.sequence.len();
        for element in &start_elements {
            let mut stepcount = 0;
            let mut current = *element;
            while current % 36 != target {
                current = if self.sequence[stepcount % self.sequence.len()] == Direction::Left {
                    self.nodetree.get(&current).unwrap().0
                } else {
                    self.nodetree.get(&current).unwrap().1
                };
                stepcount += 1;
            }
            steps = math::lcm(steps, stepcount);
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }
}
