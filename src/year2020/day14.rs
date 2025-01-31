//! # 2020 day 14 - Docking Data
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

enum Instruction {
    Mask { ones_mask: usize, x_mask: usize },
    Mem { address: usize, value: usize },
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..3] {
            "mas" => {
                let (_, right) = s.split_once(" = ").unwrap();
                let mut ones_mask = 0;
                let mut x_mask = 0;
                for (i, c) in right.chars().rev().enumerate() {
                    match c {
                        '1' => {
                            ones_mask += 1 << i;
                        }
                        'X' => {
                            x_mask += 1 << i;
                        }
                        _ => (),
                    }
                }
                Ok(Self::Mask { ones_mask, x_mask })
            }
            "mem" => {
                let (left, right) = s.split_once(" = ").unwrap();
                let left = left.trim_end_matches(']').trim_start_matches("mem[");
                Ok(Self::Mem {
                    address: left.parse().unwrap(),
                    value: right.parse().unwrap(),
                })
            }
            _ => Err(()),
        }
    }
}

struct InputData {
    instructions: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .lines()
                .map(|line| Instruction::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut memory = HashMap::new();
        let mut or_bitmask = 0;
        let mut and_bitmask = 0;
        for instr in &self.instructions {
            match instr {
                Instruction::Mask { ones_mask, x_mask } => {
                    or_bitmask = *ones_mask;
                    and_bitmask = *x_mask;
                }
                Instruction::Mem { address, value } => {
                    memory.insert(*address, (value & and_bitmask) | or_bitmask);
                }
            }
        }
        memory.values().sum()
    }

    fn solve_part2(&self) -> usize {
        let mut memory = HashMap::new();
        let mut x_bitmask = 0;
        let mut addr_masks = Vec::new();
        for instr in &self.instructions {
            match instr {
                Instruction::Mask { ones_mask, x_mask } => {
                    addr_masks.clear();
                    x_bitmask = *x_mask;
                    let mut q = VecDeque::from([(*ones_mask, x_bitmask)]);
                    while let Some((ones, x)) = q.pop_front() {
                        if x == 0 {
                            addr_masks.push(ones);
                        } else {
                            //Find the next x index
                            let mut i = 1;
                            while x != (x | i) {
                                i <<= 1;
                            }
                            let new_x = x & !i;
                            q.push_back((ones, new_x));
                            q.push_back((ones | i, new_x));
                        }
                    }
                }
                Instruction::Mem { address, value } => {
                    for amask in &addr_masks {
                        memory.insert((address & !x_bitmask) | amask, *value);
                    }
                }
            }
        }
        memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 165);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 208);
    }
}
