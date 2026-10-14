//! # 2016 day 5 - How About a Nice Game of Chess?
//!
//! Solves both parts in a single pass. They share the same sequence of
//! qualifying hashes (five leading zeroes) scanned in increasing
//! index order: part 1 takes the 6th char of each such hash, while part 2
//! uses the 6th char as a position and the 7th as its value.
use crate::aoc_util::error::AocError;
use md5::{Digest, Md5};
use std::{error::Error, fmt::Write};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok(solution_data.solve())
}

const PASSWORD_LEN: usize = 8;

struct InputData<'a> {
    door_id: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { door_id: s.trim() })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (String, String) {
        let mut part1 = String::with_capacity(PASSWORD_LEN);
        let mut part2 = ['_'; PASSWORD_LEN];
        let mut remaining = PASSWORD_LEN;

        let mut buffer = String::from(self.door_id);
        let base_len = buffer.len();
        let mut index = 0u32;

        while part1.len() < PASSWORD_LEN || remaining > 0 {
            if let Some((sixth, seventh)) = hash_candidate(&mut buffer, base_len, index) {
                if part1.len() < PASSWORD_LEN {
                    part1.push(nibble_to_char(sixth));
                }
                let position = sixth as usize;
                if position < PASSWORD_LEN && part2[position] == '_' {
                    part2[position] = nibble_to_char(seventh);
                    remaining -= 1;
                }
            }
            index += 1;
        }

        (part1, part2.iter().collect())
    }
}

/// Computes `md5(door_id + index)` reusing `buffer`, and returns the 6th and
/// 7th hex digits as `(position, value)` when the digest starts with five zeroes.
/// The prefix is checked on the raw digest bytes, avoiding the cost of
/// hex-formatting every candidate.
fn hash_candidate(buffer: &mut String, base_len: usize, index: u32) -> Option<(u8, u8)> {
    buffer.truncate(base_len);
    write!(buffer, "{index}").unwrap();
    let digest = Md5::digest(buffer.as_bytes());
    // Five leading zero nibbles: bytes 0 and 1 are zero, and byte 2's high nibble is zero.
    (digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10)
        .then(|| (digest[2] & 0x0f, digest[3] >> 4))
}

/// Converts a nibble (0..=15) into its lowercase hexadecimal character.
fn nibble_to_char(value: u8) -> char {
    char::from_digit(value as u32, 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "abc";

    // Official input, requires too many hashes to run fast enough - disabling by default
    #[test]
    #[ignore = "slow"]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve().0, "18f47a30");
    }

    // Official input, requires too many hashes to run fast enough - disabling by default
    #[test]
    #[ignore = "slow"]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve().1, "05ace8e3");
    }
}
