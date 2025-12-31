//! # 2025 day 2 - Gift Shop
//!
//! ## Part 1
//!
//! For each range,
//! 1) If the first number in the range is evenly divisible by 2, split it in two and take the first part (1234 -> 12).
//!    If not, start with the next number where it would increase the number of digits (e.g. 972 -> 1000).
//! 2) Create a new candidate id by combining the number from 1) with itself (12 -> 1212).
//! 3) If that candidate id is bigger than the second value in the range, stop.
//! 4) If that candidate id is bigger than the first value in the range, an invalid value has been found; add it to the total.
//! 5) Increment the candidate id (12 -> 13) and go back to 2).
//!
//! ## Part 2
//!
//! Pretty much same as part 1, except,
//! * Instead of just splitting in 2 parts, loop over splits in different part sizes, from 2 up to the number of digits in
//!   the second value in the range.
//! * The same number can be found with different part lengths, but should only be counted once. So the identified invalid
//!   numbers are stored in a set and then summarized at the end to get rid of duplicates.
use std::{collections::HashSet, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    ranges: Vec<(&'a str, &'a str)>,
}

fn get_invalid(v1: &str, v2: &str) -> u64 {
    let start_id: u64 = v1.parse().unwrap();
    let stop_id: u64 = v2.parse().unwrap();
    let nbrof_parts = 2;

    let mut invalid = 0;
    let mut next_part: u64 = if v1.len().is_multiple_of(nbrof_parts) {
        v1[..v1.len() / nbrof_parts].parse().unwrap()
    } else {
        10_u64.pow((v1.len() / nbrof_parts) as u32)
    };

    loop {
        let nbrof_part_digits = 1 + next_part.checked_ilog10().unwrap_or(0);
        let candidate_id = next_part + next_part * 10_u64.pow(nbrof_part_digits);
        if candidate_id > stop_id {
            break;
        }
        if candidate_id >= start_id {
            invalid += candidate_id;
        }
        next_part += 1;
    }

    invalid
}

fn get_multiple_invalid(v1: &str, v2: &str) -> u64 {
    let start_id: u64 = v1.parse().unwrap();
    let stop_id: u64 = v2.parse().unwrap();
    let mut nbrof_parts = 2;
    let mut invalid = HashSet::new();

    while nbrof_parts <= v2.len() {
        let mut next_part: u64 = if v1.len().is_multiple_of(nbrof_parts) {
            v1[..v1.len() / nbrof_parts].parse().unwrap()
        } else {
            10_u64.pow((v1.len() / nbrof_parts) as u32)
        };

        loop {
            let nbrof_part_digits = 1 + next_part.checked_ilog10().unwrap_or(0);
            let candidate_id = (0..nbrof_parts)
                .map(|p| next_part * 10_u64.pow(p as u32 * nbrof_part_digits))
                .sum();
            if candidate_id > stop_id {
                break;
            }
            if candidate_id >= start_id {
                invalid.insert(candidate_id);
            }
            next_part += 1;
        }
        nbrof_parts += 1;
    }

    invalid.iter().sum()
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            ranges: s.split(',').map(|r| r.split_once('-').unwrap()).collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> u64 {
        self.ranges.iter().map(|(r1, r2)| get_invalid(r1, r2)).sum()
    }

    fn solve_part2(&self) -> u64 {
        self.ranges
            .iter()
            .map(|(r1, r2)| get_multiple_invalid(r1, r2))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 1227775554);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 4174379265);
    }
}
