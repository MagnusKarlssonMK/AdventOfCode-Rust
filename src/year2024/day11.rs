//! # 2024 day 11 - Plutonian Pebbles
//!
//! Solves both parts in the same loop, since part 2 simply requires way more
//! iterations of the same method. Since the number of stones will quickly become
//! enormous but many will have the same value, the stones are stored in a hash-map
//! rather than a vector, with the stone's number as key and number of stones with
//! that number as value. Despite the desceptive description, the order of the stones
//! is actually not important!
use std::collections::HashMap;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve(25, 75);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    stones: Vec<usize>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            stones: input
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    fn solve(&self, blinks1: usize, blinks2: usize) -> (usize, usize) {
        let mut p1 = None;
        let mut p2 = None;
        let mut blink_counter = 0;
        let mut stones: HashMap<usize, usize> = self.stones.iter().map(|n| (*n, 1)).collect();
        while p1.is_none() || p2.is_none() {
            blink_counter += 1;
            stones = blink(&stones);
            if p1.is_none() && blink_counter == blinks1 {
                p1 = Some(stones.values().sum());
            }
            if p2.is_none() && blink_counter == blinks2 {
                p2 = Some(stones.values().sum());
            }
        }
        (p1.unwrap(), p2.unwrap())
    }
}

fn blink(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
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
                let power = 10_usize.pow(digits / 2);
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
    fn part1_example_1() {
        let testdata = String::from("0 1 10 99 999");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve(1, 1);
        assert_eq!(p1, 7);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("125 17");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve(6, 25);
        assert_eq!(p1, 22);
        assert_eq!(p2, 55312);
    }
}
