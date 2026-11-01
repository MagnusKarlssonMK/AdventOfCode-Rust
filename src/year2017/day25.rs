//! # 2017 day 25 - The Halting Problem
//!
//! Brute forcing the simulation using a hashset to store current ones on the tape. The main headache
//! is parsing the input file.
use crate::aoc_util::error::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((solution_data.solve_part1()?.to_string(), "-".to_string()))
}

#[derive(Debug)]
struct Action {
    /// 0 or 1
    write_val: u8,
    /// -1 = Left, 1 = Right
    move_val: i8,
    /// The next state
    next_state: char,
}

/// State, current value (0, 1)
type StateKey = (char, u8);

struct InputData {
    begin_state: char,
    checksum: u64,
    blueprint: HashMap<StateKey, Action>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let header = s.split("\n\n").next().ctx("header")?;
        let (h1, h2) = header.split_once('\n').ctx("checksum")?;
        let begin_state = match h1.split(' ').collect::<Vec<_>>().as_slice() {
            ["Begin", "in", "state", s] => s.chars().next().ctx("begin state")?,
            _ => {
                return Err(AocError::Invalid(h1.to_string()));
            }
        };
        let checksum = match h2.split(' ').collect::<Vec<_>>().as_slice() {
            ["Perform", "a", "diagnostic", "checksum", "after", c, _] => c.parse::<u64>()?,
            _ => {
                return Err(AocError::Invalid(h2.to_string()));
            }
        };
        let mut blueprint = HashMap::new();
        // tbd - parse the remaining blocks in parts into blueprint
        // The combination of lines "In state X:" and "If the current value is Y:" gives the StateKey (X, Y), where Y will be 0 or 1
        // The chunk of three lines in between are used to get the Action.
        let mut current_state: Option<char> = None;
        let mut current_value: Option<u8> = None;
        let mut write_val: Option<u8> = None;
        let mut move_val: Option<i8> = None;

        for line in s.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            // Skip header lines
            if trimmed.starts_with("Begin in state") || trimmed.starts_with("Perform a diagnostic")
            {
                continue;
            }

            if let Some(rest) = trimmed.strip_prefix("In state ") {
                current_state = rest.trim_end_matches(':').chars().next();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("If the current value is ") {
                current_value = rest.trim_end_matches(':').parse::<u8>().ok();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("- Write the value ") {
                write_val = rest.trim_end_matches('.').parse::<u8>().ok();
                continue;
            }
            if trimmed.contains("Move one slot to the left") {
                move_val = Some(-1);
                continue;
            }
            if trimmed.contains("Move one slot to the right") {
                move_val = Some(1);
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("- Continue with state ") {
                let next_state = rest
                    .trim_end_matches('.')
                    .chars()
                    .next()
                    .ctx("next state")?;
                if let (Some(st), Some(val), Some(w), Some(m)) =
                    (current_state, current_value, write_val, move_val)
                {
                    blueprint.insert(
                        (st, val),
                        Action {
                            write_val: w,
                            move_val: m,
                            next_state,
                        },
                    );
                }
                // Reset for next branch
                write_val = None;
                move_val = None;
                continue;
            }
        }
        Ok(Self {
            begin_state,
            checksum,
            blueprint,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<usize, AocError> {
        let mut tape_ones = HashSet::new();
        let mut current_pos: i64 = 0;
        let mut current_state = self.begin_state;
        let mut current_val = 0u8;
        for _ in 0..self.checksum {
            let action = self
                .blueprint
                .get(&(current_state, current_val))
                .ok_or(AocError::NoSolution)?;
            if action.write_val == 1 {
                tape_ones.insert(current_pos);
            } else {
                tape_ones.remove(&current_pos);
            }
            current_pos += action.move_val as i64;
            current_state = action.next_state;
            current_val = if tape_ones.contains(&current_pos) {
                1
            } else {
                0
            };
        }
        Ok(tape_ones.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 3);
    }
}
