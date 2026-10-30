//! # 2017 day 23 - Coprocessor Conflagration
//!
//! Simply implementing the emulator here won't work; the assemply is far too inefficient
//! to run within reasonable time. So instead the input is disassembled and analyzed manually,
//! and the solution is implementing the corresponding calculation after extracting the
//! relevant numbers from the input.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

/// Determines whether a certain number using the 6k+-1 rule.
fn is_prime(n: u64) -> bool {
    // Numbers less than or equal to 1 are not prime
    if n <= 1 {
        return false;
    }

    // 2 and 3 are prime numbers
    if n <= 3 {
        return true;
    }

    // Multiples of 2 or 3 are not prime
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    // Check factors of the form 6k +- 1 up to sqrt(n)
    let mut i = 5;
    while i * i <= n {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }

    true
}

struct InputData {
    b_initial: u64,
    mul_b: u64,
    sub_b_1: u64,
    sub_b_2: u64,
    sub_c: u64,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let mut set_b = None;
        let mut mul_b = None;
        let mut sub_b_1 = None;
        let mut sub_b_2 = None;
        let mut sub_c = None;
        for line in lines {
            match line.split(' ').collect::<Vec<_>>().as_slice() {
                ["set", "b", n] => set_b = Some(n.parse()?),
                ["mul", "b", n] => mul_b = Some(n.parse()?),
                ["sub", "b", n] => {
                    if sub_b_1.is_none() {
                        sub_b_1 = Some(n.parse::<i64>()?.unsigned_abs());
                    } else {
                        sub_b_2 = Some(n.parse::<i64>()?.unsigned_abs());
                    }
                }
                ["sub", "c", n] => sub_c = Some(n.parse::<i64>()?.unsigned_abs()),
                _ => (),
            }
        }
        Ok(Self {
            b_initial: set_b.ok_or(AocError::Missing("set b"))?,
            mul_b: mul_b.ok_or(AocError::Missing("mul b"))?,
            sub_b_1: sub_b_1.ok_or(AocError::Missing("sub b 1"))?,
            sub_b_2: sub_b_2.ok_or(AocError::Missing("sub b 2"))?,
            sub_c: sub_c.ok_or(AocError::Missing("sub c"))?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        let mul_count = self.b_initial - 2;
        mul_count * mul_count
    }

    fn solve_part2(&self) -> usize {
        let start = self.mul_b * self.b_initial + self.sub_b_1;
        let end = start + self.sub_c;
        let step = self.sub_b_2 as usize;
        (start..=end)
            .step_by(step)
            .filter(|n| !is_prime(*n))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CUSTOM_1: &str = "set b 5
sub b 7
mul b 3
set a 1
sub c 4
sub b 3";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 9);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
