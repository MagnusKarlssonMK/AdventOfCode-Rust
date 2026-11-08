//! # 2020 day 15 - Rambunctious Recitation
//!
//! Basically the Van Eck sequence. Seems to require brute-forcing, so part 2
//! is really slow and uses a lot of RAM to keep track of the counters for each
//! number. Use u32 rather than usize to at least keep it somewhat under control.
use crate::aoc_util::error::AocError;
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    start_list: Vec<u32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            start_list: s.split(',').map(|n| n.parse()).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn play(&self, rounds: u32) -> u32 {
        let (last, earlier) = self
            .start_list
            .split_last()
            .expect("start list cannot be empty");
        let start_len = self.start_list.len() as u32;
        if rounds <= start_len {
            return self.start_list[rounds as usize - 1];
        }

        let mut nbrs: HashMap<u32, u32> = earlier
            .iter()
            .enumerate()
            .map(|(i, &n)| (n, i as u32))
            .collect();

        let mut lastspoken = *last;
        for turn in start_len..rounds {
            let speak = match nbrs.insert(lastspoken, turn - 1) {
                Some(n) => turn - n - 1,
                None => 0,
            };
            lastspoken = speak;
        }
        lastspoken
    }

    fn solve_part1(&self) -> u32 {
        self.play(2020)
    }

    fn solve_part2(&self) -> u32 {
        self.play(30_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = "0,3,6".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 436);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = "1,3,2".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = "2,1,3".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
    }

    #[test]
    fn part1_official_4() {
        let solution_data: InputData = "1,2,3".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 27);
    }

    #[test]
    fn part1_official_5() {
        let solution_data: InputData = "2,3,1".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 78);
    }

    #[test]
    fn part1_official_6() {
        let solution_data: InputData = "3,2,1".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 438);
    }

    #[test]
    fn part1_official_7() {
        let solution_data: InputData = "3,1,2".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1836);
    }

    // Skip part 2 test, since it's really slow and anyway just a lot more iterations
    // of the same function.

    #[test]
    fn part1_duplicate_start_values() {
        let solution_data: InputData = "0,0,1".parse().unwrap();
        assert_eq!(solution_data.play(10), 6);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_start_list_longer_than_rounds() {
        let input = (1..=2024)
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let solution_data: InputData = input.parse().unwrap();
        assert_eq!(solution_data.play(2020), 2020);
        assert_eq!(solution_data.play(2024), 2024);
    }
}
