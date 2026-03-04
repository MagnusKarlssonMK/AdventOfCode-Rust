//! # 2015 day 11 - Corporate Policy
//!
//! Kind of a brute force solution, simply incrementing the password until it reaches
//! a valid value.
//!
//! There could be optimizations made to increment in larger interval chunks by evaluating
//! the current password more intelligently. But the current solution is still decently fast
//! on modern hardware.
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1, p2))
}

const OFFSET_VAL: u8 = b'a';
const RANGE_VAL: u8 = b'z' - OFFSET_VAL;
const FORBIDDEN_VALS: [u8; 3] = [b'i' - OFFSET_VAL, b'l' - OFFSET_VAL, b'o' - OFFSET_VAL];

struct InputData {
    current_password: Vec<u8>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| !c.is_ascii_lowercase()) {
            Err(())
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

    #[test]
    fn invalid_input() {
        let testdata = "abcDefgh";
        assert!(InputData::from_str(testdata).is_err());

        let testdata = "abc{efgh";
        assert!(InputData::from_str(testdata).is_err());
    }

    #[test]
    fn validate_example_1() {
        let testdata = "hijklmmn";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert!(!is_password_valid(&solution_data.current_password));
    }

    #[test]
    fn validate_example_2() {
        let testdata = "abbceffg";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert!(!is_password_valid(&solution_data.current_password));
    }

    #[test]
    fn validate_example_3() {
        let testdata = "abbcegjk";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert!(!is_password_valid(&solution_data.current_password));
    }

    #[test]
    fn part1_example_1() {
        let testdata = "abcdefgh";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, "abcdffaa");
    }

    #[test]
    fn part1_example_2() {
        let testdata = "ghijklmn";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, "ghjaabcc");
    }
}
