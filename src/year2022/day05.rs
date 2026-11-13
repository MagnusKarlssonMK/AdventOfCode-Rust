//! # 2022 day 5 - Supply Stacks
//!
//! Stacks are 1-indexed in the input, so subtract 1 from every move instruction to get the correct
//! index with a stack stored in regular 0-indexing double-vector. After parsing, running the
//! procedures is straightforward, just some vector trickery.
//!
//! Note that this is one of those puzzles with trailing spaces in the input file in the crates
//! part which gets shaved off automatically by editors, so the parsing needs to handle uneven line
//! lengths, and not rely on empty spaces.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((solution_data.solve_part1()?, solution_data.solve_part2()?))
}

struct Procedure {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Procedure {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["move", a, "from", f, "to", t] => {
                let amount = a.parse()?;
                let mut from = f.parse()?;
                let mut to = t.parse()?;
                if from == 0 || to == 0 {
                    return Err(AocError::Invalid(s.to_string()));
                }
                // Convert from 1-indexed to 0-indexed
                from -= 1;
                to -= 1;
                Ok(Self { amount, from, to })
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

struct InputData {
    crates: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, p) = s
            .split_once("\n\n")
            .ctx("separator between crate map and procedures missing")?;
        let crate_lines: Vec<_> = c.lines().collect();
        // Note: the actual characters in the bottom line is always 1,2,3,... So we can ignore the
        // actual characters and only record the index for each character, and the crates in the
        // lines above will be on that same index.
        let crate_indexes: Vec<usize> = crate_lines
            .last()
            .ctx("crate map is empty")?
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(|(i, _)| i)
            .collect();
        let mut crates = vec![Vec::new(); crate_indexes.len()];
        for line in crate_lines.iter().rev().skip(1) {
            for (i, c) in line.chars().enumerate() {
                for (j, c_i) in crate_indexes.iter().enumerate() {
                    if c_i == &i && c != ' ' {
                        if !c.is_ascii_uppercase() {
                            return Err(AocError::Invalid(format!("crate map invalid char: {c}")));
                        }
                        crates[j].push(c);
                    }
                }
            }
        }
        Ok(Self {
            crates,
            procedures: p
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<String, AocError> {
        self.run_procedures(false)
    }

    fn solve_part2(&self) -> Result<String, AocError> {
        self.run_procedures(true)
    }

    fn run_procedures(&self, multi_crates: bool) -> Result<String, AocError> {
        let mut crates = self.crates.clone();
        let mut buffer = Vec::new();
        for p in &self.procedures {
            let source = crates.get_mut(p.from).ok_or_else(|| {
                AocError::Invalid(format!("source stack {} out of range", p.from + 1))
            })?;
            let split = source.len().checked_sub(p.amount).ok_or_else(|| {
                AocError::Invalid(format!(
                    "cannot move {} crates from stack {} with only {} crates",
                    p.amount,
                    p.from + 1,
                    source.len()
                ))
            })?;
            buffer.extend(source.drain(split..));
            let target = crates.get_mut(p.to).ok_or_else(|| {
                AocError::Invalid(format!("target stack {} out of range", p.to + 1))
            })?;
            if multi_crates {
                target.extend(buffer.iter());
            } else {
                target.extend(buffer.iter().rev());
            }
            buffer.clear();
        }
        crates
            .iter()
            .map(|c| c.last().copied().ok_or(AocError::NoSolution))
            .collect::<Result<_, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), "CMZ");
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), "MCD");
    }

    #[test]
    fn procedures_reject_out_of_range_stacks() {
        for input in [
            " A\n 1   2\n\nmove 1 from 1 to 3",
            " A\n 1   2\n\nmove 1 from 3 to 1",
        ] {
            let data: InputData = input.parse().unwrap();
            assert!(matches!(data.solve_part1(), Err(AocError::Invalid(_))));
        }
    }

    #[test]
    fn procedures_reject_overdraw() {
        let data: InputData = " A\n 1   2\n\nmove 2 from 1 to 2".parse().unwrap();
        assert!(matches!(data.solve_part1(), Err(AocError::Invalid(_))));
    }

    #[test]
    fn parser_invalid_procedures() {
        for input in [
            " 1   2\n\nmove 1 from 1",        // missing tokens
            " 1   2\n\nmove x from 1 to 2",   // non-numeric amount
            " 1   2\n\nmove 1 from 0 to 2",   // zero index
            " 1   2\n\nmove 1 from 1 to 2 x", // extra token
        ] {
            assert!(input.parse::<InputData>().is_err());
        }
    }
}
