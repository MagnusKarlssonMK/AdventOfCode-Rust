//! # 2022 day 5 - Supply Stacks
//!
//! ## Parsing
//!
//! Stacks are 1-indexed in the input, so subtract 1 from every move instruction
//! to get the correct index with a stack stored in regular 0-indexing double-vector.
//! After parsing, running the procedures is straightforward, just some vector
//! trickery.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((solution_data.solve_part1(), solution_data.solve_part2()))
}

struct Procedure {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Procedure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        tokens.next();
        let amount = tokens.next().unwrap().parse().unwrap();
        tokens.next();
        let from = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        tokens.next();
        let to = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        Ok(Self { amount, from, to })
    }
}

struct InputData {
    crates: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, p) = s.split_once("\n\n").unwrap();
        let c_lines: Vec<_> = c.lines().collect();
        let c_indices: Vec<usize> = c_lines
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(|(i, _)| i)
            .collect();
        let mut crates = vec![Vec::new(); c_indices.len()];
        for line in c_lines.iter().rev().skip(1) {
            for (i, c) in line.chars().enumerate() {
                for (j, c_i) in c_indices.iter().enumerate() {
                    if c_i == &i && c != ' ' {
                        crates[j].push(c);
                    }
                }
            }
        }
        Ok(Self {
            crates,
            procedures: p
                .lines()
                .map(|line| Procedure::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> String {
        self.run_procedures(false)
    }

    fn solve_part2(&self) -> String {
        self.run_procedures(true)
    }

    fn run_procedures(&self, multi_crates: bool) -> String {
        let mut crates = self.crates.clone();
        let mut buffer = Vec::new();
        for p in self.procedures.iter() {
            let i = crates[p.from].len() - p.amount;
            buffer.extend(crates[p.from].drain(i..));
            if !multi_crates {
                crates[p.to].extend(buffer.iter().rev());
            } else {
                crates[p.to].extend(buffer.iter());
            }
            buffer.clear();
        }
        crates.iter().map(|c| c.last().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), "CMZ");
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), "MCD");
    }
}
