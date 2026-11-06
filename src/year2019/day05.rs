//! # 2019 day 5 - Sunny with a Chance of Asteroids
use crate::aoc_util::error::AocError;

use super::intcode::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    program: Vec<isize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s.split(',').map(|c| c.parse()).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<isize, AocError> {
        self.get_diagnostic(1)
    }

    fn solve_part2(&self) -> Result<isize, AocError> {
        self.get_diagnostic(5)
    }

    fn get_diagnostic(&self, instruction: isize) -> Result<isize, AocError> {
        let mut cpu = Intcode::new(&self.program);
        loop {
            let state = cpu.run();
            match state {
                State::Input => {
                    cpu.input_value(instruction);
                }
                State::Output(v) => {
                    if v != 0 {
                        return Ok(v);
                    }
                }
                State::Halted => return Err(AocError::NoSolution),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "3,0,4,0,99";

    const TEST_OFFICIAL_2: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    /// Input that generates no output for part 2
    const TEST_CUSTOM_2: &str = "3,9,8,9,10,9,4,9,99,-1,8";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 1);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.get_diagnostic(7).unwrap(), 999);
        assert_eq!(solution_data.get_diagnostic(8).unwrap(), 1000);
        assert_eq!(solution_data.get_diagnostic(9).unwrap(), 1001);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }
}
