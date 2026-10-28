//! # 2017 day 16 - Permutation Promenade
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1(),
        solution_data.solve_part2(PART_2_ROUNDS),
    ))
}

#[derive(Debug)]
enum DanceMove {
    /// sX, makes X programs move from the end to the front, but maintain their order otherwise.
    Spin(usize),
    /// xA/B, makes the programs at positions A and B swap places.
    Exchange(usize, usize),
    /// pA/B, makes the programs named A and B swap places.
    Partner(char, char),
}

impl FromStr for DanceMove {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        match c.next() {
            Some('s') => Ok(DanceMove::Spin(c.collect::<String>().parse()?)),
            Some('x') => {
                let rest = c.collect::<String>();
                let (left, right) = rest.split_once('/').ctx("x dash separator")?;
                Ok(DanceMove::Exchange(left.parse()?, right.parse()?))
            }
            Some('p') => {
                let rest = c.collect::<String>();
                let (left, right) = rest.split_once('/').ctx("x dash separator")?;
                let a = left.chars().next().ctx("p A")?;
                let b = right.chars().next().ctx("p B")?;
                Ok(DanceMove::Partner(a, b))
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

const PROGRAMS: &str = "abcdefghijklmnop";
const PART_2_ROUNDS: usize = 1_000_000_000;

#[derive(Debug)]
struct InputData {
    dance_moves: Vec<DanceMove>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            dance_moves: s
                .split(',')
                .map(|dance| dance.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn perform_dance(&self, programs: &mut [char]) {
        for dance_move in &self.dance_moves {
            let nbr_programs = programs.len();
            match dance_move {
                DanceMove::Spin(v) => programs.rotate_right(*v),
                DanceMove::Exchange(a, b) => programs.swap(*a % nbr_programs, *b % nbr_programs),
                DanceMove::Partner(a, b) => {
                    for p in programs.iter_mut() {
                        if p == a {
                            *p = *b;
                        } else if p == b {
                            *p = *a;
                        }
                    }
                }
            }
        }
    }

    fn solve_part1(&self) -> String {
        let mut programs: Vec<_> = PROGRAMS.chars().collect();
        self.perform_dance(&mut programs);
        programs.iter().collect()
    }

    fn solve_part2(&self, rounds: usize) -> String {
        let mut programs: Vec<_> = PROGRAMS.chars().collect();
        let mut cycle_len = None;
        let mut orders = Vec::from([PROGRAMS.to_string()]);
        while cycle_len.is_none() && orders.len() <= rounds {
            self.perform_dance(&mut programs);
            let s: String = programs.iter().collect();
            if s == orders[0] {
                cycle_len = Some(orders.len());
            }
            orders.push(s);
        }
        if let Some(c) = cycle_len {
            orders[rounds % c].to_string()
        } else {
            // Reached the round target without ever repeating : Just return the latest order
            // `orders` is guaranteed to have at least one element, so unwrap will never panic
            orders.last().unwrap().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "s1,x3/4,pe/b";

    const TEST_CUSTOM_1: &str = "s7,x0/1,x2/15";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), "paedcbfghijklmno");
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(PART_2_ROUNDS), "ghidjklmnopabcef");
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        // This input has cycle length of 140, so with 100 rounds this will test the "no cycle found" branch
        assert_eq!(solution_data.solve_part2(100), "dbcfehgkijmlonap");
    }
}
