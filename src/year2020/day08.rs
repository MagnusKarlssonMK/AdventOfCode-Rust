//! # 2020 day 8 - Handheld Halting
//!
//! Nothing too fancy, basically brute force that runs fast anyway.
//! For part 2 just try swapping the instructions one by one until it works.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Clone)]
enum Instruction {
    Jmp(isize),
    Acc(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').unwrap();
        let right: isize = right.parse().unwrap();
        match left {
            "jmp" => Ok(Self::Jmp(right)),
            "acc" => Ok(Self::Acc(right)),
            "nop" => Ok(Self::Nop(right)),
            _ => Err(()),
        }
    }
}

enum ProgramState {
    Loop(isize),
    Halted(isize),
}

struct InputData {
    program: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s
                .lines()
                .map(|line| Instruction::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn run_program(&self) -> ProgramState {
        let p_len: isize = self.program.len() as isize;
        let mut seen = vec![false; p_len as usize];
        let mut accumulator = 0;
        let mut pc: isize = 0;
        loop {
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
            if !(0..p_len).contains(&pc) {
                panic!(); // Should never happen with correct input
            }
        }
    }

    fn solve_part1(&self) -> usize {
        match self.run_program() {
            ProgramState::Loop(v) => v as usize,
            _ => 0,
        }
    }

    fn solve_part2(&mut self) -> usize {
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
                return a as usize;
            }
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part2_example_1() {
        let mut solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 8);
    }
}
