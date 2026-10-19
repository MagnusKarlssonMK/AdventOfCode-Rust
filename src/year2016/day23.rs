//! # 2016 day 23 - Safe Cracking
//!
//! Uses emulator for the Assembunny program, which is split off to a separate
//! module since it is basically identical with what's used in day 12, only with
//! the addition of the 'tgl' opcode.
//!
//! The program is doing addition and multiplication with huge numbers, but only
//! has +1 operations available, causing a ridiculous number of iterations. So
//! instead, optimize the program by creating two new operations, overwriting it
//! where necessary and then run the program. This assumes that the new 'tgl'
//! operation doesn't try to toggle anything within these blocks, which the real
//! input doesn't seem to do.
use super::assembunny::*;
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    program: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut program = s
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        optimize_program(&mut program);
        Ok(Self { program })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<i32, AocError> {
        let mut reg = Registers::default();
        reg.set_reg('a', 7);
        run_program(&self.program, &mut reg)?;
        reg.get_reg('a')
    }

    fn solve_part2(&self) -> Result<i32, AocError> {
        let mut reg = Registers::default();
        reg.set_reg('a', 12);
        run_program(&self.program, &mut reg)?;
        reg.get_reg('a')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 3);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 3);
    }
}
