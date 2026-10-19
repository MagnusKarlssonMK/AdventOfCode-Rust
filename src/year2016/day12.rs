//! # 2016 day 12 - Leonardo's Monorail
//!
//! Uses emulator for the Assembunny program, which is split off to a separate
//! module since it is reused in day 23.
//!
//! The optimizer needed for day 23 can be used here as well, and although it
//! isn't nearly as critical for this day, it still does cut down the execution
//! time noticeably.
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
        run_program(&self.program, &mut reg)?;
        reg.get_reg('a')
    }

    fn solve_part2(&self) -> Result<i32, AocError> {
        let mut reg = Registers::default();
        reg.set_reg('c', 1);
        run_program(&self.program, &mut reg)?;
        reg.get_reg('a')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 42);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 42);
    }
}
