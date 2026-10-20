//! # 2016 day 15 - Timing is Everything
//!
//! Disc d (numbered from 1) has n positions and starts at position p, so it is at the goal at time t when
//! (p + t + d) % n == 0, i.e. t == -(p + d) (mod n). The input's position counts are pairwise coprime, so the
//! button-press time is the Chinese remainder of those congruences. Part 2 adds an extra 11-position disc
//! starting at position 0.
//!
//! Any input where the position counts are not pairwise coprime will result in `AocError::NoSolution`, since
//! the Chinese Remainder calculation will not find any answer.
use crate::aoc_util::{error::*, math::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(Debug)]
struct Disc {
    nbr_positions: usize,
    start_position: usize,
}

#[derive(Debug)]
struct InputData {
    discs: Vec<Disc>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut discs = Vec::new();
        for line in s.lines() {
            let words: Vec<_> = line.split_ascii_whitespace().collect();
            let start_position: usize = words
                .get(11)
                .ctx("start pos")?
                .trim_end_matches('.')
                .parse()?;
            let nbr_positions: usize = words.get(3).ctx("nbr pos")?.parse()?;
            discs.push(Disc {
                nbr_positions,
                start_position,
            });
        }
        Ok(Self { discs })
    }
}

impl InputData {
    fn get_buttonpress_time(&self, extra_disc: bool) -> Result<i128, AocError> {
        // Part 2 adds disc #(len + 1) with 11 positions starting at position 0.
        let extra = Disc {
            nbr_positions: 11,
            start_position: 0,
        };
        let discs = self.discs.iter().chain(extra_disc.then_some(&extra));

        let mut moduli = Vec::with_capacity(self.discs.len() + extra_disc as usize);
        let mut remainders = Vec::with_capacity(self.discs.len() + extra_disc as usize);
        for (i, disc) in discs.enumerate() {
            let n = disc.nbr_positions;
            moduli.push(n as i128);
            remainders.push(((n - (disc.start_position + i + 1) % n) % n) as i128);
        }

        chinese_remainder(&moduli, &remainders).ok_or(AocError::NoSolution)
    }

    fn solve_part1(&self) -> Result<i128, AocError> {
        self.get_buttonpress_time(false)
    }

    fn solve_part2(&self) -> Result<i128, AocError> {
        self.get_buttonpress_time(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    const TEST_CUSTOM_1: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.";

    const TEST_CUSTOM_2: &str = "Disc #1 has 2 positions; at time=0, it is at position 1.
Disc #2 has 3 positions; at time=0, it is at position 0.
Disc #3 has 5 positions; at time=0, it is at position 2.";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 5);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 0);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 10);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 85);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 20);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 40);
    }

    #[test]
    fn parser_short_input() {
        let broken_text = "Disc #1 has 5a positions; at time=0, it is at";
        let err = InputData::from_str(broken_text).unwrap_err();
        assert_eq!(err, AocError::Missing("start pos"));
    }

    #[test]
    fn parser_invalid_integer() {
        let broken_text = "Disc #1 has 5a positions; at time=0, it is at position 4.";
        let err = InputData::from_str(broken_text).unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
