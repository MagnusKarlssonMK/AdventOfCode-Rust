//! # 2024 day 11 - Plutonian Pebbles
//!
//! Solves both parts in the same loop, since part 2 simply requires way more iterations of the
//! same method. Since the number of stones will quickly become enormous but many will have the
//! same value, the stones are stored in a hash-map rather than a vector, with the stone's number
//! as key and number of stones with that number as value. Despite the desceptive description, the
//! order of the stones is actually not important!
use crate::aoc_util::error::AocError;
use std::{collections::HashMap, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve(25, 75);
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    stones: Vec<u64>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            stones: s
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self, blinks1: usize, blinks2: usize) -> (u64, u64) {
        let mut stones: HashMap<u64, u64> = HashMap::new();
        for n in &self.stones {
            stones.entry(*n).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut p1 = stones.values().sum();
        let mut p2 = p1;

        for blink_counter in 1..=blinks1.max(blinks2) {
            stones = blink(&stones);
            if blink_counter == blinks1 {
                p1 = stones.values().sum();
            }
            if blink_counter == blinks2 {
                p2 = stones.values().sum();
            }
        }

        (p1, p2)
    }
}

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut newstones = HashMap::new();
    for (stone, val) in stones.iter() {
        if *stone == 0 {
            newstones
                .entry(1)
                .and_modify(|v| *v += *val)
                .or_insert(*val);
        } else {
            let digits = 1 + stone.ilog10();
            if digits % 2 == 0 {
                let power = 10u64.pow(digits / 2);
                newstones
                    .entry(stone / power)
                    .and_modify(|v| *v += *val)
                    .or_insert(*val);
                newstones
                    .entry(stone % power)
                    .and_modify(|v| *v += *val)
                    .or_insert(*val);
            } else {
                newstones
                    .entry(stone * 2024)
                    .and_modify(|v| *v += *val)
                    .or_insert(*val);
            }
        }
    }
    newstones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = "0 1 10 99 999".parse().unwrap();
        let (p1, _) = solution_data.solve(1, 1);
        assert_eq!(p1, 7);
    }

    #[test]
    fn parts1_2_official_2() {
        let solution_data: InputData = "125 17".parse().unwrap();
        let (p1, p2) = solution_data.solve(6, 25);
        assert_eq!(p1, 22);
        assert_eq!(p2, 55312);
    }

    #[test]
    fn parts1_2_duplicate_stones() {
        let solution_data: InputData = "1 1".parse().unwrap();
        let (p1, p2) = solution_data.solve(1, 1);
        assert_eq!(p1, 2);
        assert_eq!(p2, 2);
    }
}
