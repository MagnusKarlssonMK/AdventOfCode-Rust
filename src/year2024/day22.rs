//! # 2024 day 22 - Monkey Market
//!
//! Part 1 evolves each secret 2000 times and sums the results. Part 2 records, for each buyer,
//! the banana price at the first occurrence of every 4-change price sequence, then takes the
//! largest total across buyers. Buyers are processed in parallel with per-worker maps that are
//! merged afterwards; sequences are packed into `u32` keys to avoid allocating per change.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    thread::*,
};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    secret_numbers: Vec<u64>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            secret_numbers: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Deltas are in -9..=9, so four of them pack into a `u32` base-19 key (19^4 = 130_321).
type Bananas = HashMap<u32, isize>;

impl InputData {
    fn solve(&self) -> Result<(u64, isize), AocError> {
        let results: Vec<(u64, Bananas)> = spawn_jobs(self.secret_numbers.clone(), |numbers| {
            let mut bananas = Bananas::new();
            let total: u64 = numbers
                .into_iter()
                .map(|secret| process_secret(secret, &mut bananas))
                .sum();
            (total, bananas)
        });

        let p1: u64 = results.iter().map(|(total, _)| *total).sum();
        let mut bananas = Bananas::new();
        for (_, partial) in results {
            for (key, value) in partial {
                *bananas.entry(key).or_default() += value;
            }
        }
        let p2 = bananas.into_values().max().ctx("no banana sequences")?;
        Ok((p1, p2))
    }
}

fn process_secret(secret: u64, bananas: &mut Bananas) -> u64 {
    let mut seen = HashSet::new();
    let mut changes = [0isize; 4];
    let mut next = secret;
    let mut last_price = (next % 10) as isize;
    for change_count in 0..2000 {
        next = evolve(next);
        let price = (next % 10) as isize;
        let change = price - last_price;
        if change_count < 4 {
            changes[change_count] = change;
        } else {
            changes = [changes[1], changes[2], changes[3], change];
            let key = changes
                .iter()
                .fold(0u32, |key, &d| key * 19 + (d + 9) as u32);
            if seen.insert(key) {
                *bananas.entry(key).or_default() += price;
            }
        }
        last_price = price;
    }
    next
}

fn evolve(mut nbr: u64) -> u64 {
    nbr = (nbr ^ (nbr << 6)) % 16777216;
    nbr = (nbr ^ (nbr >> 5)) % 16777216;
    (nbr ^ (nbr << 11)) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "1
10
100
2024";

    const TEST_OFFICIAL_2: &str = "1
2
3
2024";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, _) = solution_data.solve().unwrap();
        assert_eq!(p1, 37327623);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (_, p2) = solution_data.solve().unwrap();
        assert_eq!(p2, 23);
    }
}
