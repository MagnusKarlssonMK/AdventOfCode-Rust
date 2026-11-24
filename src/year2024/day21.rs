//! # 2024 day 21 - Keypad Conundrum
//!
//! For each code, walk the numpad from `A` to each digit and record the required moves, then
//! expand that sequence through the directional keypad twice (part 1) or 25 times (part 2).
//! Part 2 counts repeated subsequences in a `HashMap` so the exponentially growing sequences
//! never need to be materialised. Each final length is multiplied by the code's numeric value.
use crate::aoc_util::error::{AocError, OptionExt};
use crate::aoc_util::point::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const NUMPAD: &str = "789\n456\n123\n.0A";
const DIRPAD: &str = ".^A\n<v>";

struct Keypad {
    keys: HashMap<char, Point>,
    positions: HashMap<Point, char>,
}

impl Keypad {
    fn new(s: &str) -> Self {
        let mut keys = HashMap::new();
        let mut positions = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_, ch)| *ch != '.') {
                keys.insert(c, Point::new(x as i32, y as i32));
                positions.insert(Point::new(x as i32, y as i32), c);
            }
        }
        Self { keys, positions }
    }

    fn get_commands(&self, path: &[char]) -> Result<Vec<char>, AocError> {
        let mut result = Vec::new();
        let mut current = 'A';
        for next in path.iter() {
            result.extend(self.get_step(current, *next)?);
            current = *next;
        }
        Ok(result)
    }

    fn get_commands_counter(&self, path: &[char]) -> Result<HashMap<Vec<char>, u64>, AocError> {
        let mut result = HashMap::new();
        let mut current = 'A';
        for next in path.iter() {
            let subpath = self.get_step(current, *next)?;
            result.entry(subpath).and_modify(|e| *e += 1).or_insert(1);
            current = *next;
        }
        Ok(result)
    }

    fn get_step(&self, from: char, to: char) -> Result<Vec<char>, AocError> {
        let from_point = self.keys.get(&from).ctx(format!("unknown key: {from}"))?;
        let to_point = self.keys.get(&to).ctx(format!("unknown key: {to}"))?;
        let delta = *to_point - *from_point;
        let mut x_keys = match delta.x.cmp(&0) {
            Ordering::Greater => vec!['>'; delta.x.unsigned_abs() as usize],
            Ordering::Less => vec!['<'; delta.x.unsigned_abs() as usize],
            Ordering::Equal => Vec::new(),
        };
        let mut y_keys = match delta.y.cmp(&0) {
            Ordering::Greater => vec!['v'; delta.y.unsigned_abs() as usize],
            Ordering::Less => vec!['^'; delta.y.unsigned_abs() as usize],
            Ordering::Equal => Vec::new(),
        };
        if delta.x > 0
            && self
                .positions
                .contains_key(&Point::new(from_point.x, to_point.y))
        {
            y_keys.extend(x_keys);
            y_keys.extend(['A']);
            Ok(y_keys)
        } else if self
            .positions
            .contains_key(&Point::new(to_point.x, from_point.y))
        {
            x_keys.extend(y_keys);
            x_keys.extend(['A']);
            Ok(x_keys)
        } else {
            y_keys.extend(x_keys);
            y_keys.extend(['A']);
            Ok(y_keys)
        }
    }
}

struct Code {
    keys: Vec<char>,
    value: u64,
}

struct InputData {
    codes: Vec<Code>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codes = s
            .lines()
            .map(|line| {
                let digits = line.strip_suffix('A').ctx("code must end with A")?;
                if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
                    return Err(AocError::Invalid(format!("invalid code: {line}")));
                }
                Ok(Code {
                    keys: line.chars().collect(),
                    value: digits.parse::<u64>()?,
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { codes })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<u64, AocError> {
        let num_pad = Keypad::new(NUMPAD);
        let dir_pad = Keypad::new(DIRPAD);

        let mut seqs: Vec<Vec<char>> = self
            .codes
            .iter()
            .map(|code| num_pad.get_commands(&code.keys))
            .collect::<Result<_, _>>()?;
        for _ in 0..2 {
            seqs = seqs
                .iter()
                .map(|code| dir_pad.get_commands(code))
                .collect::<Result<_, _>>()?;
        }
        Ok(seqs
            .iter()
            .zip(&self.codes)
            .map(|(seq, code)| seq.len() as u64 * code.value)
            .sum())
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let num_pad = Keypad::new(NUMPAD);
        let dir_pad = Keypad::new(DIRPAD);
        let num_seqs: Vec<Vec<char>> = self
            .codes
            .iter()
            .map(|code| num_pad.get_commands(&code.keys))
            .collect::<Result<_, _>>()?;
        let mut seq_counter: Vec<HashMap<Vec<char>, u64>> = num_seqs
            .iter()
            .map(|s| HashMap::from([(s.clone(), 1)]))
            .collect();
        for _ in 0..25 {
            let mut new_seqs = Vec::new();
            for rc in seq_counter {
                let mut new_seq = HashMap::new();
                for (sub_seq, qty) in rc {
                    for (k, v) in dir_pad.get_commands_counter(&sub_seq)? {
                        new_seq
                            .entry(k)
                            .and_modify(|e| *e += v * qty)
                            .or_insert(v * qty);
                    }
                }
                new_seqs.push(new_seq);
            }
            seq_counter = new_seqs;
        }
        Ok(seq_counter
            .iter()
            .zip(&self.codes)
            .map(|(seq, code)| {
                seq.iter().map(|(k, v)| k.len() as u64 * v).sum::<u64>() * code.value
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 126384);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 154115708116294);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["029X", "029", "\n", "0A9A"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
    }
}
