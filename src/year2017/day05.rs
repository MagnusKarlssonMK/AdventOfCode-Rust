//! # 2017 day 5 - Maze of Twisty Trampolines, All Alike
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    instructions: Vec<i32>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s.lines().map(|i| i.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn run_program(&self, strange: bool) -> usize {
        let mut program = self.instructions.to_vec();
        let mut sp: i32 = 0;
        let mut count: usize = 0;
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

    const TEST_DATA: &str = "0\n3\n0\n1\n-3";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 10);
    }
}
