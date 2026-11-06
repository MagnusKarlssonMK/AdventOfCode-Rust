//! # 2019 day 2 - 1202 Program Alarm
use crate::aoc_util::error::AocError;

use super::intcode::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const P2_TARGET: isize = 19690720;

struct InputData {
    program: Vec<isize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s.split(',').map(|n| n.parse()).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> isize {
        let mut cpu = Intcode::new(&self.program);
        cpu.overwrite_pos(1, 12);
        cpu.overwrite_pos(2, 2);
        cpu.run();
        cpu.read_pos(0)
    }

    fn solve_part2(&self) -> Result<isize, AocError> {
        // Try every permitted noun/verb combination.
        for noun in 0..100 {
            for verb in 0..100 {
                let mut cpu = Intcode::new(&self.program);
                cpu.overwrite_pos(1, noun);
                cpu.overwrite_pos(2, verb);
                cpu.run();
                if cpu.read_pos(0) == P2_TARGET {
                    return Ok(100 * noun + verb);
                }
            }
        }
        Err(AocError::NoSolution)
    }
}
