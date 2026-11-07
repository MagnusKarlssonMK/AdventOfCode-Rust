//! # 2020 day 8 - Handheld Halting
//!
//! Nothing too fancy, basically brute force that runs fast anyway.
//! For part 2 just try swapping the instructions one by one until it works.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(Clone)]
enum Instruction {
    Jmp(isize),
    Acc(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').ctx("instruction separator")?;
        let right: isize = right.parse()?;
        match left {
            "jmp" => Ok(Self::Jmp(right)),
            "acc" => Ok(Self::Acc(right)),
            "nop" => Ok(Self::Nop(right)),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

enum ProgramState {
    Loop(isize),
    Halted(isize),
    OutOfBounds,
}

struct InputData {
    program: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn run_program(&self) -> ProgramState {
        let p_len: isize = self.program.len() as isize;
        let mut seen = vec![false; p_len as usize];
        let mut accumulator = 0;
        let mut pc: isize = 0;
        while (0..p_len).contains(&pc) {
            if seen[pc as usize] {
                return ProgramState::Loop(accumulator);
            }
            seen[pc as usize] = true;
            match self.program[pc as usize] {
                Instruction::Jmp(v) => {
                    pc += v;
                }
                Instruction::Acc(v) => {
                    accumulator += v;
                    pc += 1;
                }
                Instruction::Nop(_) => {
                    pc += 1;
                }
            }
            if pc == p_len {
                return ProgramState::Halted(accumulator);
            }
        }
        ProgramState::OutOfBounds
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        match self.run_program() {
            ProgramState::Loop(v) => Ok(v as usize),
            _ => Err(AocError::NoSolution),
        }
    }

    fn solve_part2(&mut self) -> Result<usize, AocError> {
        for i in 0..self.program.len() {
            let instr = self.program[i].clone();
            match instr {
                Instruction::Jmp(v) => self.program[i] = Instruction::Nop(v),
                Instruction::Nop(v) => self.program[i] = Instruction::Jmp(v),
                Instruction::Acc(_) => continue,
            }
            let result = self.run_program();
            self.program[i] = instr;
            if let ProgramState::Halted(a) = result {
                return Ok(a as usize);
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    /// No nop/jmp to swap, so the program can never be repaired.
    const TEST_CUSTOM_1: &str = "acc +1
acc +2";

    const TEST_CUSTOM_2: &str = "nop +100
acc +2
jmp +0
acc +3
acc +1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 5);
    }

    #[test]
    fn part2_official_1() {
        let mut solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 8);
    }

    #[test]
    fn part2_custom_1() {
        let mut solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        let err = solution_data.solve_part2().unwrap_err();
        assert_eq!(err, AocError::NoSolution);
    }

    #[test]
    fn part2_custom_2() {
        let mut solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 6);
    }
}
