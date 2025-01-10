//! # 2019 day 5 - Sunny with a Chance of Asteroids
use super::intcode::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    program: Vec<isize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s.split(',').map(|c| c.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> isize {
        self.get_diagnostic(1)
    }

    fn solve_part2(&self) -> isize {
        self.get_diagnostic(5)
    }

    fn get_diagnostic(&self, instruction: isize) -> isize {
        let mut cpu = Intcode::new(&self.program);
        loop {
            let state = cpu.run();
            match state {
                State::Input => {
                    cpu.input_value(instruction);
                }
                State::Output(v) => {
                    if v != 0 {
                        break v;
                    }
                }
                State::Halted => break 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.get_diagnostic(7), 999);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.get_diagnostic(8), 1000);
    }

    #[test]
    fn part2_example_3() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.get_diagnostic(9), 1001);
    }
}
