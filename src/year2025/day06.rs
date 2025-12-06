//! # 2025 day 6 - Trash Compactor
//!
//! A parsing exercise.
//! The difference between part 1 and part 2 is how to parse the numbers; horizontally or
//! vertically. It would probably make a lot more sense to simply transpose the input for
//! parsing it vertically, than the absolute mess I ended up with here.
use std::{error::Error, str::FromStr};

use crate::aoc_util::{grid::Grid, point::Point};

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
    numbers: Vec<usize>,
    number_cols: Vec<usize>,
    operation: Operation,
}

impl Column {
    fn simple_eval(&self) -> usize {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Mul => self.numbers.iter().product(),
        }
    }

    fn cephalopod_eval(&self) -> usize {
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
        let value_lines: Vec<Vec<usize>> = nbrs
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
                number_cols: vec![0; value_lines.len()],
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
        let mut op_idxs: Vec<_> = op
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(|(i, _)| i)
            .collect();

        // Find the vertical numbers
        let nbr_grid = Grid::parse(nbrs);
        op_idxs.push(nbr_grid.x_max + 1);
        let col_ranges: Vec<(usize, usize)> =
            op_idxs.windows(2).map(|w| (w[0], w[1] - 1)).collect();
        for (i, (col_start, col_stop)) in col_ranges.iter().enumerate() {
            let mut nbrs: Vec<usize> = Vec::new();
            for x in *col_start..*col_stop {
                let mut c_vec = Vec::new();
                for y in 0..value_lines.len() {
                    let c = nbr_grid
                        .get_element(&Point::new(x as i32, y as i32))
                        .unwrap();
                    if c != ' ' {
                        c_vec.push(c);
                    }
                }
                let nbr: String = c_vec.iter().collect();
                nbrs.push(nbr.parse().unwrap());
            }
            columns[i].number_cols = nbrs;
        }
        Ok(Self { columns })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.columns.iter().map(|c| c.simple_eval()).sum()
    }

    fn solve_part2(&self) -> usize {
        self.columns.iter().map(|c| c.cephalopod_eval()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: need to format the input this way, since rust formatter otherwise shaves off whitespaces
    // at the end of lines, which are essential for parsing line length and mapping to a grid.
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
