//! # 2025 day 6 - Trash Compactor
//!
//! A parsing exercise.
//! Group the input in column groups, separated by empty column so each group gets one
//! operation ('+' or '*') connected to it.
//!
//! The difference between part 1 and part 2 is how to then parse the numbers; horizontally or
//! vertically. Parsing horizontally is done with trivial whitespace splitting. The vertical
//! numbers are derived by first transposing the input and then treating the empty lines as
//! separators for the column groups.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Column {
    numbers: Vec<u64>,
    number_cols: Vec<u64>,
    operation: Operation,
}

impl Column {
    fn simple_eval(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Mul => self.numbers.iter().product(),
        }
    }

    fn cephalopod_eval(&self) -> u64 {
        match self.operation {
            Operation::Add => self.number_cols.iter().sum(),
            Operation::Mul => self.number_cols.iter().product(),
        }
    }
}

struct InputData {
    columns: Vec<Column>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nbrs, op) = s.rsplit_once('\n').unwrap();
        let operations: Vec<Operation> = op
            .split_whitespace()
            .map(|s| Operation::from_str(s).unwrap())
            .collect();

        let value_lines: Vec<Vec<u64>> = nbrs
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();
        let mut columns = vec![
            Column {
                numbers: vec![0; value_lines.len()],
                number_cols: Vec::new(),
                operation: Operation::Add,
            };
            value_lines[0].len()
        ];

        // Find the horizontal numbers
        for (i, line) in value_lines.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                columns[j].numbers[i] = *c;
            }
        }
        for (i, o) in operations.iter().enumerate() {
            columns[i].operation = *o;
        }

        // Find the vertical numbers by first transposing the input lines
        let lines: Vec<Vec<char>> = nbrs.lines().map(|line| line.chars().collect()).collect();
        let mut lines_transposed = vec![vec![' '; lines.len()]; lines[0].len()];
        for row in 0..lines.len() {
            for col in 0..lines[0].len() {
                lines_transposed[col][row] = lines[row][col];
            }
        }

        let mut col_idx = 0;
        for line in lines_transposed.iter() {
            if let Ok(val) = line.iter().collect::<String>().trim().parse::<u64>() {
                columns[col_idx].number_cols.push(val);
            } else {
                col_idx += 1;
            }
        }

        Ok(Self { columns })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.columns.iter().map(|c| c.simple_eval()).sum()
    }

    fn solve_part2(&self) -> u64 {
        self.columns.iter().map(|c| c.cephalopod_eval()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: need to format the input this way, since rust formatter otherwise shaves off whitespaces
    // at the end of lines, which are essential for parsing lines correctly.
    const TEST_DATA: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 4277556);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 3263827);
    }
}
