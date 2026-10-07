//! # 2015 day 11 - Corporate Policy
//!
//! Kind of a brute force solution, simply incrementing the password until it reaches
//! a valid value.
//!
//! There could be optimizations made to increment in larger interval chunks by evaluating
//! the current password more intelligently. But the current solution is still decently fast
//! on modern hardware.
use crate::aoc_util::error::AocError;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1, p2))
}

const OFFSET_VAL: u8 = b'a';
const RANGE_VAL: u8 = b'z' - OFFSET_VAL;
const FORBIDDEN_VALS: [u8; 3] = [b'i' - OFFSET_VAL, b'l' - OFFSET_VAL, b'o' - OFFSET_VAL];

#[derive(Debug)]
struct InputData {
    current_password: Vec<u8>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| !c.is_ascii_lowercase()) {
            Err(AocError::Invalid("invalid char".to_string()))
        } else if s.len() < 5 {
            Err(AocError::Invalid("too short".to_string()))
        } else {
            Ok(Self {
                current_password: s.chars().map(|c| c as u8 - OFFSET_VAL).collect(),
            })
        }
    }
}

/// Evaluates a password to see if it fulfills the validity conditions.
fn is_password_valid(pwd: &[u8]) -> bool {
    if pwd.iter().any(|v| FORBIDDEN_VALS.contains(v)) {
        return false;
    }
    let mut rule_one = false;
    for w in pwd.windows(3) {
        if w[1] == w[0] + 1 && w[2] == w[1] + 1 {
            rule_one = true;
            break;
        }
    }
    if !rule_one {
        return false;
    }
    let mut rule_three = HashSet::new();
    for w in pwd.windows(2) {
        if w[0] == w[1] {
            rule_three.insert(w[0]);
        }
    }
    rule_three.len() > 1
}

/// Recursively generates the next possible password
fn get_next_password(pwd: &[u8]) -> Vec<u8> {
    if let Some(old_last) = pwd.last() {
        if *old_last == RANGE_VAL {
            let mut prefix = get_next_password(&pwd[..pwd.len() - 1]);
            prefix.push(0);
            prefix
        } else {
            let mut new_pwd = pwd.to_vec();
            let new_last = new_pwd.last_mut().unwrap();
            *new_last += 1;
            if FORBIDDEN_VALS.contains(new_last) {
                *new_last += 1;
            }
            new_pwd
        }
    } else {
        // Should never happen, unless we start from a password above the last possible one and we've tried to wrap around the first value
        Vec::new()
    }
}

impl InputData {
    fn solve(&self) -> (String, String) {
        let mut pwd1 = self.current_password.clone();
        while !is_password_valid(&pwd1) {
            pwd1 = get_next_password(&pwd1);
        }
        let mut pwd2 = get_next_password(&pwd1);
        while !is_password_valid(&pwd2) {
            pwd2 = get_next_password(&pwd2);
        }
        (
            pwd1.iter().map(|v| (v + OFFSET_VAL) as char).collect(),
            pwd2.iter().map(|v| (v + OFFSET_VAL) as char).collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "abcdefgh";
    const TEST_OFFICIAL_2: &str = "ghijklmn";
    const TEST_CUSTOM_1: &str = "zzzzz";
    const TEST_CUSTOM_2: &str = "aabcck";
    const TEST_CUSTOM_3: &str = "zzzzzzzy";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "abcdffaa");
        assert_eq!(p2, "abcdffbb");
    }

    #[test]
    fn part1_2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "ghjaabcc");
        assert_eq!(p2, "ghjbbcdd");
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "aabcc");
        assert_eq!(p2, "bbcdd");
    }

    #[test]
    fn part1_2_custom_3() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "aabcck");
        assert_eq!(p2, "aabccm");
    }

    #[test]
    fn part1_2_custom_4() {
        let solution_data = InputData::from_str(TEST_CUSTOM_3).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "aaaaabcc");
        assert_eq!(p2, "aaaabbcd");
    }

    #[test]
    fn parse_invalid_input() {
        let err = InputData::from_str("abcDefgh").unwrap_err();
        assert_eq!(err, AocError::Invalid("invalid char".to_string()));

        let err = InputData::from_str("abc{efgh").unwrap_err();
        assert_eq!(err, AocError::Invalid("invalid char".to_string()));

        let err = InputData::from_str("abce").unwrap_err();
        assert_eq!(err, AocError::Invalid("too short".to_string()));
    }
}
