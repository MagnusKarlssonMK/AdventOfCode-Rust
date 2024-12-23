//! # 2024 day 22 - Monkey Market
//!
//! Really slow part 2...
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    secret_numbers: Vec<usize>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            secret_numbers: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut bananas: HashMap<Vec<isize>, isize> = HashMap::new();
        let p1 = self
            .secret_numbers
            .iter()
            .map(|&n| {
                let mut seen = HashSet::new();
                let mut prices = Vec::with_capacity(2001);
                let mut changes = Vec::new();
                let mut next = n;
                prices.push((next % 10) as isize);
                for _ in 0..2000 {
                    next = evolve(next);
                    let next_10th = (next % 10) as isize;
                    if changes.len() < 4 {
                        changes.push(next_10th - prices.last().unwrap());
                    } else {
                        changes.rotate_left(1);
                        changes[3] = next_10th - prices.last().unwrap();
                        if seen.insert(changes.clone()) {
                            *bananas.entry(changes.clone()).or_default() += next_10th;
                        }
                    }
                    prices.push(next_10th);
                }
                next
            })
            .sum();
        (p1, *bananas.values().max().unwrap() as usize)
    }
}

fn evolve(mut nbr: usize) -> usize {
    nbr = (nbr ^ (nbr << 6)) % 16777216;
    nbr = (nbr ^ (nbr >> 5)) % 16777216;
    (nbr ^ (nbr << 11)) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "1
10
100
2024",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 37327623);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "1
2
3
2024",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 23);
    }
}
