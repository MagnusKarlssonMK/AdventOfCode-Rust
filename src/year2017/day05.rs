//! # 2017 day 5 - Maze of Twisty Trampolines, All Alike
//!
//! Straightforward; just step through the program with a stackpointer, while modifying
//! the program along the way as described in the instructions. The only difference
//! between part 1 and 2 is the step for the program modification at each jump, although
//! this takes much longer to run.
use std::{error::Error, str::FromStr};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    instructions: Vec<i32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .lines()
                .map(|i| i.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn run_program(&self, strange: bool) -> usize {
        let mut program = self.instructions.to_vec();
        let mut sp: i32 = 0;
        let mut count = 0;
        let program_length = program.len() as i32;
        while (0..program_length).contains(&sp) {
            count += 1;
            let idx = sp as usize;
            let val = program[idx];
            if strange && val >= 3 {
                program[idx] -= 1;
            } else {
                program[idx] += 1;
            }
            sp += val;
        }
        count
    }

    fn solve_part1(&self) -> usize {
        self.run_program(false)
    }

    fn solve_part2(&self) -> usize {
        self.run_program(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "0\n3\n0\n1\n-3";

    const TEST_CUSTOM_1: &str = "2\n-1\n-2\n-2";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 10);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 13);
    }
}
