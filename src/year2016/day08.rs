//! # 2016 day 8 - Two-Factor Authentication
use crate::aoc_util::{grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2))
}

enum Operation {
    Rect(u8, u8),
    RotateRow(u8, u8),
    RotateCol(u8, u8),
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match tokens.next().unwrap() {
            "rect" => {
                let (x, y) = tokens.next().unwrap().split_once('x').unwrap();
                Ok(Self::Rect(x.parse().unwrap(), y.parse().unwrap()))
            }
            "rotate" => {
                tokens.next();
                let (a, b) = tokens.next().unwrap().split_once('=').unwrap();
                tokens.next();
                let c: u8 = tokens.next().unwrap().parse().unwrap();
                match a {
                    "x" => Ok(Self::RotateCol(b.parse().unwrap(), c)),
                    "y" => Ok(Self::RotateRow(b.parse().unwrap(), c)),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

impl Grid {
    fn perform_operation(&mut self, op: &Operation) {
        match op {
            Operation::Rect(x_len, y_len) => {
                for x in 0..*x_len {
                    for y in 0..*y_len {
                        self.set_point(&Point::new(x as i32, y as i32), '#');
                    }
                }
            }
            Operation::RotateRow(y, n) => {
                let mut swapbuf = Vec::with_capacity(self.x_max);
                for x in 0..self.x_max as i32 {
                    swapbuf.push(
                        self.get_element(&Point::new(
                            (self.x_max as i32 + x - *n as i32) % self.x_max as i32,
                            *y as i32,
                        ))
                        .unwrap(),
                    );
                }
                for x in 0..self.x_max as i32 {
                    self.set_point(&Point::new(x, *y as i32), swapbuf[x as usize]);
                }
            }
            Operation::RotateCol(x, n) => {
                let mut swapbuf = Vec::with_capacity(self.y_max);
                for y in 0..self.y_max as i32 {
                    swapbuf.push(
                        self.get_element(&Point::new(
                            *x as i32,
                            (self.y_max as i32 + y - *n as i32) % self.y_max as i32,
                        ))
                        .unwrap(),
                    );
                }
                for y in 0..self.y_max as i32 {
                    self.set_point(&Point::new(*x as i32, y), swapbuf[y as usize]);
                }
            }
        }
    }
}

struct InputData {
    ops: Vec<Operation>,
    width: usize,
    height: usize,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            ops: s
                .lines()
                .map(|line| Operation::from_str(line).unwrap())
                .collect(),
            width: 50,
            height: 6,
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, String) {
        let mut screen = Grid::new(self.width, self.height, '.');
        for op in &self.ops {
            screen.perform_operation(op);
        }
        (
            screen.elements.iter().filter(|e| **e == '#').count(),
            "\n".to_string() + &screen.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata =
            "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1";
        let mut solution_data = InputData::from_str(testdata).unwrap();
        solution_data.width = 7;
        solution_data.height = 3;
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, "\n.#..#.#\n#.#....\n.#.....\n");
    }
}
