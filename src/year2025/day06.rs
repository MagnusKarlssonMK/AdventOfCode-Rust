//! # 2025 day 6 - Trash Compactor
//!
//! Pretty much a parsing exercise.
//!
//! Group the input in column groups, separated by empty column so each group gets one operation
//! ('+' or '*') connected to it.
//!
//! The difference between part 1 and part 2 is how to then parse the numbers; horizontally or
//! vertically. Parsing horizontally is done with trivial whitespace splitting. The vertical
//! numbers are derived by treating each run of digit columns as a number. Missing cells beyond a
//! line whose trailing spaces were trimmed are treated as spaces, so the parser does not depend
//! on that whitespace being preserved.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
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
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(AocError::Invalid(format!("invalid operation: {s}"))),
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
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<&str> = s.lines().collect();
        let op_line = lines.pop().ctx("missing operator line")?;
        let operations: Vec<Operation> = op_line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        let width = lines.iter().map(|line| line.len()).max().ctx("no value lines")?;

        let mut columns: Vec<Column> = operations
            .iter()
            .copied()
            .map(|operation| Column {
                numbers: Vec::with_capacity(lines.len()),
                number_cols: Vec::new(),
                operation,
            })
            .collect();

        // Horizontal numbers: group n is the n-th whitespace-separated token of every row.
        for line in &lines {
            let tokens: Vec<_> = line.split_whitespace().collect();
            if tokens.len() != columns.len() {
                return Err(AocError::Invalid(format!(
                    "expected {} numbers per row, got {}",
                    columns.len(),
                    tokens.len()
                )));
            }
            for (column, token) in columns.iter_mut().zip(tokens) {
                column.numbers.push(token.parse()?);
            }
        }

        // Vertical numbers: a run of columns containing digits is one number. Columns past the
        // end of a trimmed line count as spaces.
        let mut vertical: Vec<Vec<u64>> = Vec::with_capacity(columns.len());
        let mut current = Vec::new();
        let mut in_number = false;
        for col in 0..width {
            let mut value = 0u64;
            let mut has_digit = false;
            for line in &lines {
                if let Some(&b) = line.as_bytes().get(col)
                    && b != b' '
                {
                    let digit = b
                        .checked_sub(b'0')
                        .filter(|d| *d <= 9)
                        .ctx("invalid digit in number block")?;
                    value = value
                        .checked_mul(10)
                        .and_then(|v| v.checked_add(digit as u64))
                        .ctx("vertical number too large")?;
                    has_digit = true;
                }
            }
            if !has_digit {
                if in_number {
                    vertical.push(std::mem::take(&mut current));
                    in_number = false;
                }
                continue;
            }
            in_number = true;
            current.push(value);
        }
        if in_number {
            vertical.push(current);
        }
        if vertical.len() != columns.len() {
            return Err(AocError::Invalid(format!(
                "expected {} number groups, got {}",
                columns.len(),
                vertical.len()
            )));
        }
        for (column, values) in columns.iter_mut().zip(vertical) {
            column.number_cols = values;
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

    // Trailing spaces deliberately stripped; the parser must not depend on them.
    const TEST_OFFICIAL_1: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    const TEST_OFFICIAL_PADDED: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4277556);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 3263827);
    }

    #[test]
    fn padded_and_trimmed_inputs_agree() {
        let trimmed: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let padded: InputData = TEST_OFFICIAL_PADDED.parse().unwrap();
        assert_eq!(trimmed.solve_part1(), padded.solve_part1());
        assert_eq!(trimmed.solve_part2(), padded.solve_part2());
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["+", "123 456", "123 456\n+ -", "123 456\n+"] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
    }
}
