//! # 2024 day 21 - Keypad Conundrum
use crate::aoc_util::point::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const NUMPAD: &str = "789\n456\n123\n.0A";
const DIRPAD: &str = ".^A\n<v>";

struct Keypad {
    keys: HashMap<char, Point>,
    positions: HashMap<Point, char>,
}

impl FromStr for Keypad {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut keys = HashMap::new();
        let mut positions = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_, ch)| *ch != '.') {
                keys.insert(c, Point::new(x as i32, y as i32));
                positions.insert(Point::new(x as i32, y as i32), c);
            }
        }
        Ok(Self { keys, positions })
    }
}

impl Keypad {
    fn get_commands(&self, path: &[char]) -> Vec<char> {
        let mut result = Vec::new();
        let mut current = 'A';
        for next in path.iter() {
            for c in self.get_step(current, *next) {
                result.push(c);
            }
            current = *next;
        }
        result
    }

    fn get_commands_counter(&self, path: &[char]) -> HashMap<Vec<char>, usize> {
        let mut result = HashMap::new();
        let mut current = 'A';
        for next in path.iter() {
            let mut subpath = Vec::new();
            for c in self.get_step(current, *next) {
                subpath.push(c);
            }
            result.entry(subpath).and_modify(|e| *e += 1).or_insert(1);
            current = *next;
        }
        result
    }

    fn get_step(&self, from: char, to: char) -> Vec<char> {
        let from_point = self.keys.get(&from).unwrap();
        let to_point = self.keys.get(&to).unwrap();
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
            y_keys
        } else if self
            .positions
            .contains_key(&Point::new(to_point.x, from_point.y))
        {
            x_keys.extend(y_keys);
            x_keys.extend(['A']);
            x_keys
        } else {
            y_keys.extend(x_keys);
            y_keys.extend(['A']);
            y_keys
        }
    }
}

struct InputData {
    codes: Vec<Vec<char>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            codes: s.lines().map(|line| line.chars().collect()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let num_pad = Keypad::from_str(NUMPAD).unwrap();
        let dir_pad = Keypad::from_str(DIRPAD).unwrap();

        let mut seqs: Vec<Vec<char>> = self
            .codes
            .iter()
            .map(|code| num_pad.get_commands(code))
            .collect();
        for _ in 0..2 {
            seqs = seqs.iter().map(|code| dir_pad.get_commands(code)).collect();
        }
        seqs.iter()
            .zip(&self.codes)
            .map(|(seq, code)| seq.len() * code_to_num(code))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let num_pad = Keypad::from_str(NUMPAD).unwrap();
        let dir_pad = Keypad::from_str(DIRPAD).unwrap();
        let num_seqs: Vec<Vec<char>> = self
            .codes
            .iter()
            .map(|code| num_pad.get_commands(code))
            .collect();
        let mut seq_counter = Vec::new();
        num_seqs.iter().for_each(|s| {
            seq_counter.push(HashMap::from([(s.clone(), 1)]));
        });
        for _ in 0..25 {
            let mut new_seqs = Vec::new();
            for rc in seq_counter {
                let mut new_seq = HashMap::new();
                for (sub_seq, qty) in rc {
                    let new_seq_counter = dir_pad.get_commands_counter(&sub_seq);
                    new_seq_counter.iter().for_each(|(k, v)| {
                        new_seq
                            .entry(k.clone())
                            .and_modify(|e| *e += *v * qty)
                            .or_insert(*v * qty);
                    });
                }
                new_seqs.push(new_seq);
            }
            seq_counter = new_seqs;
        }
        seq_counter
            .iter()
            .zip(&self.codes)
            .map(|(seq, code)| {
                seq.iter().map(|(k, v)| k.len() * v).sum::<usize>() * code_to_num(code)
            })
            .sum()
    }
}

fn code_to_num(input: &[char]) -> usize {
    let s: String = input[0..input.len() - 1].iter().collect();
    s.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 126384);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 154115708116294);
    }
}
