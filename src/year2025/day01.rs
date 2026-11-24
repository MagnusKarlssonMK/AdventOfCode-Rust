//! # 2025 day 1 - Secret Entrance
//!
//! ## Part 1
//!
//! * Store the input as a vector of signed integer values.
//! * Then for all input values, use modulo operation to keep updating the dial value, but we need
//!   to use the rem_euclid function for correct result when the sum of the dial and the next value
//!   is negative.
//! * Increment counter whenever the dial value is zero.
//!
//! ## Part 2
//!
//! For all input values,
//! * If the input value is positive, increment the counter with (dial + input value) / 100, i.e.
//!   the number of times the dial will wrap around.
//! * Else if the dial is currently zero, increment the counter with the abs(input value) / 100. We
//!   need to handle this case separately to avoid counting the current zero dial value twice.
//! * Else, the input value is negative, so we need to "count backwards"; increment the counter
//!   with (100 - dial + abs(input value)) / 100.
//! * Then update the dial with the same modulo operation as in part 1.
//!
//! A possible alternative solution for part 2 could be to keep track of the current direction
//! instead of working with signed integers, and flip the dial around whenever the direction
//! changes. That way, all changes to the dial will be in the positive direction, i.e. there would
//! be no need to handle "left" (negative) values separately.
use crate::aoc_util::error::AocError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Lock {
    dial: i32,
}

const LOCK_SIZE: i32 = 100;

impl Lock {
    fn new() -> Self {
        Self { dial: 50 }
    }

    fn rotate_and_count(&mut self, value: i32) -> u32 {
        self.dial = (self.dial + value).rem_euclid(LOCK_SIZE);
        if self.dial == 0 { 1 } else { 0 }
    }

    fn rotate_and_count_any(&mut self, value: i32) -> u32 {
        let magnitude = value.unsigned_abs();
        let dial = self.dial as u32;
        let size = LOCK_SIZE as u32;
        let count = if value >= 0 {
            (dial + magnitude) / size
        } else if dial == 0 {
            magnitude / size
        } else {
            (size - dial + magnitude) / size
        };
        self.dial = (self.dial + value).rem_euclid(LOCK_SIZE);
        count
    }
}

fn parse_step(s: &str) -> Result<i32, AocError> {
    let (direction, value) = s
        .split_at_checked(1)
        .ok_or_else(|| AocError::Invalid(format!("invalid step: {s}")))?;
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(AocError::Invalid(format!("invalid step: {s}")));
    }
    let value: i32 = value.parse()?;
    match direction {
        "L" => Ok(-value),
        "R" => Ok(value),
        _ => Err(AocError::Invalid(format!("invalid step: {s}"))),
    }
}

struct InputData {
    sequence: Vec<i32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sequence: s.lines().map(parse_step).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u32 {
        let mut lock = Lock::new();
        self.sequence
            .iter()
            .map(|v| lock.rotate_and_count(*v))
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        let mut lock = Lock::new();
        self.sequence
            .iter()
            .map(|v| lock.rotate_and_count_any(*v))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn direction_signs() {
        let left: InputData = "L1".parse().unwrap();
        assert_eq!(left.sequence, [-1]);
        let right: InputData = "R1".parse().unwrap();
        assert_eq!(right.sequence, [1]);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["", "X10", "L", "Lx", "L+5", "L 5", "L-5"] {
            assert!(
                matches!(parse_step(input), Err(AocError::Invalid(_))),
                "expected Invalid for {input:?}"
            );
        }
        assert!(matches!(
            "L68\nX10".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }
}
