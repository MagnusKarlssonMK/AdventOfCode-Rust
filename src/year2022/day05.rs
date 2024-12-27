//! # 2022 day 5 - Supply Stacks
//!
//! ## Parsing
//!
//! Stacks are 1-indexed in the input, so subtract 1 from every move instruction
//! to get the correct index with a stack stored in regular 0-indexing double-vector.
//! After parsing, running the procedures is straightforward, just some vector
//! trickery.
pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
struct Procedure {
    amount: usize,
    from: usize,
    to: usize,
}

impl Procedure {
    fn new(input: &str) -> Self {
        let mut tokens = input.split_whitespace();
        tokens.next();
        let amount = tokens.next().unwrap().parse().unwrap();
        tokens.next();
        let from = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        tokens.next();
        let to = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        Self { amount, from, to }
    }
}

struct InputData {
    crates: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let (c, p) = input.split_once("\n\n").unwrap();
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
        Self {
            crates,
            procedures: p.lines().map(Procedure::new).collect(),
        }
    }

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

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), "CMZ");
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), "MCD");
    }
}
