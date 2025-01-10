//! # 2019 day 2 - 1202 Program Alarm
use super::intcode::*;
use std::{cmp::Ordering, error::Error, str::FromStr};

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
            program: s.split(',').map(|n| n.parse().unwrap()).collect(),
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

    fn solve_part2(&self) -> isize {
        const TARGET: isize = 19690720;
        let mut noun_max = 99;
        let mut noun_min = 0;
        let mut verb_max = 99;
        let mut verb_min = 0;
        let mut noun = 0;
        let mut verb = 0;
        loop {
            //Since we so far only have add and mul op-codes, changes in the noun/verb will have a linear impact on the resulting value.
            //I.e. noun/verb functions like arms on a clock and we can use binary search to dial in the answer rather than brute-forcing 0-99.
            let mut cpu = Intcode::new(&self.program);
            cpu.overwrite_pos(1, noun);
            cpu.overwrite_pos(2, verb);
            cpu.run();
            match cpu.read_pos(0).cmp(&TARGET) {
                Ordering::Equal => break 100 * noun + verb,
                Ordering::Greater => {
                    noun_max = noun - 1;
                    if noun > noun_min {
                        noun = (noun - 1 + noun_min) / 2;
                    } else {
                        verb_max = verb - 1;
                        verb = (verb - 1 + verb_min) / 2;
                    }
                }
                Ordering::Less => {
                    noun_min = noun + 1;
                    if noun < noun_max {
                        noun = (noun + 1 + noun_max) / 2;
                    } else {
                        verb_min = verb + 1;
                        verb = (verb + 1 + verb_max) / 2;
                    }
                }
            }
        }
    }
}
