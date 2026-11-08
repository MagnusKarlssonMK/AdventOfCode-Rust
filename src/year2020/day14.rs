//! # 2020 day 14 - Docking Data
//!
//! ## Part 1
//!
//! A bitmask is applied to the value: mask-1 bits are set to 1, mask-0 bits are set to 0, and X
//! bits are copied from the value.
//!
//! ## Part 2
//!
//! The mask is applied to the address instead: mask-1 bits are forced on, mask-0 bits keep the
//! original address bit, and every floating (X) bit is expanded into all combinations by
//! enumerating the submasks of the floating bit positions. The solution assumes that at least one
//! `mask` instruction will appear prior to the first `mem` instruction; any `mem` instruction
//! before that will be silently ignored.
//!
//! The expansion is exponential in the number of X bits: a mask with `n` floating bits produces
//! `2^n` address masks, so a hypothetical 36-X mask would generate `2^36` entries. This is not
//! capped; the solver assumes the number of X bits per mask in the input is small, which seems
//! to hold for all real inputs.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mask { ones_mask: u64, x_mask: u64 },
    Mem { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" = ").ctx("instruction =")?;
        if left == "mask" {
            let mut ones_mask = 0;
            let mut x_mask = 0;
            for (i, c) in right.chars().rev().enumerate() {
                if i >= u64::BITS as usize {
                    return Err(AocError::Invalid(s.to_string()));
                }
                match c {
                    '1' => ones_mask |= 1 << i,
                    'X' => x_mask |= 1 << i,
                    '0' => (),
                    _ => return Err(AocError::Invalid(s.to_string())),
                }
            }
            Ok(Self::Mask { ones_mask, x_mask })
        } else if let Some(address) = left.strip_prefix("mem[").and_then(|a| a.strip_suffix(']')) {
            Ok(Self::Mem {
                address: address.parse()?,
                value: right.parse()?,
            })
        } else {
            Err(AocError::Invalid(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    instructions: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
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

    fn solve_part2(&self) -> u64 {
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
                            let i = 1u64 << x.trailing_zeros();
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

    const TEST_OFFICIAL_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const TEST_OFFICIAL_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 165);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 208);
    }

    #[test]
    fn parser_invalid_instruction() {
        assert!("m = 1".parse::<Instruction>().is_err());
        assert!("mask = 2".parse::<Instruction>().is_err());
        assert!("mem[1 = 2".parse::<Instruction>().is_err());
        assert!("memory[1] = 2".parse::<Instruction>().is_err());
        assert!(
            format!("mask = {}", "X".repeat(65))
                .parse::<Instruction>()
                .is_err()
        );
    }

    #[test]
    fn parser_valid_instruction() {
        assert_eq!(
            "mem[8] = 11".parse::<Instruction>().unwrap(),
            Instruction::Mem {
                address: 8,
                value: 11,
            }
        );
    }
}
