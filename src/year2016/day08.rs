//! # 2016 day 8 - Two-Factor Authentication
use crate::aoc_util::{
    error::{AocError, OptionExt},
    grid::*,
    point::*,
};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2))
}

#[derive(Debug)]
enum Operation {
    Rect(u8, u8),
    RotateRow(u8, u8),
    RotateCol(u8, u8),
}

impl FromStr for Operation {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match tokens.next() {
            Some("rect") => {
                let (x, y) = tokens
                    .next()
                    .ctx("rect dimensions")?
                    .split_once('x')
                    .ctx("rect x separator")?;
                Ok(Self::Rect(x.parse()?, y.parse()?))
            }
            Some("rotate") => {
                let axis = tokens.next().ctx("rotate axis")?;
                let (coord, value) = tokens
                    .next()
                    .ctx("rotate coord")?
                    .split_once('=')
                    .ctx("rotate = separator")?;
                tokens.next().ctx("rotate by")?;
                let amount: u8 = tokens.next().ctx("rotate amount")?.parse()?;
                match (axis, coord) {
                    ("column", "x") => Ok(Self::RotateCol(value.parse()?, amount)),
                    ("row", "y") => Ok(Self::RotateRow(value.parse()?, amount)),
                    _ => Err(AocError::Invalid(s.to_string())),
                }
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Grid<char> {
    fn perform_operation(&mut self, op: &Operation) -> Result<(), AocError> {
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
                    if let Some(val) = self.get_element(&Point::new(
                        (self.x_max as i32 + x - *n as i32) % self.x_max as i32,
                        *y as i32,
                    )) {
                        swapbuf.push(val);
                    } else {
                        return Err(AocError::Invalid(format!("row {y} out of bounds")));
                    }
                }
                for x in 0..self.x_max as i32 {
                    self.set_point(&Point::new(x, *y as i32), swapbuf[x as usize]);
                }
            }
            Operation::RotateCol(x, n) => {
                let mut swapbuf = Vec::with_capacity(self.y_max);
                for y in 0..self.y_max as i32 {
                    if let Some(val) = self.get_element(&Point::new(
                        *x as i32,
                        (self.y_max as i32 + y - *n as i32) % self.y_max as i32,
                    )) {
                        swapbuf.push(val);
                    } else {
                        return Err(AocError::Invalid(format!("col {x} out of bounds")));
                    }
                }
                for y in 0..self.y_max as i32 {
                    self.set_point(&Point::new(*x as i32, y), swapbuf[y as usize]);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct InputData {
    ops: Vec<Operation>,
    width: usize,
    height: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            ops: s
                .lines()
                .map(Operation::from_str)
                .collect::<Result<_, _>>()?,
            width: 50,
            height: 6,
        })
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, String), AocError> {
        let mut screen = Grid::new(self.width, self.height, &'.');
        for op in &self.ops {
            screen.perform_operation(op)?;
        }
        Ok((
            screen.elements.iter().filter(|e| **e == '#').count(),
            "\n".to_string() + &screen.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    const TEST_CUSTOM_1: &str = "rect 9x4
rotate column x=0 by 1
rotate row y=0 by 0
rotate column x=8 by 1
rotate row y=3 by 0";

    const ANSWER_P2_CUSTOM_1: &str = "\n.#######..........................................
#########.........................................
#########.........................................
#########.........................................
#.......#.........................................
..................................................\n";

    #[test]
    fn part1_2_official_1() {
        let mut solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        solution_data.width = 7;
        solution_data.height = 3;
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 6);
        assert_eq!(p2, "\n.#..#.#\n#.#....\n.#.....\n");
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 36);
        assert_eq!(p2, ANSWER_P2_CUSTOM_1);
    }

    #[test]
    fn parser_invalid_int() {
        for input in [
            "rect 3f2x44",
            "rect 32x4f4",
            "rotate row y=1 by 3f",
            "rotate row y=1e by 3",
        ] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::ParseInt(_))
            ))
        }
    }

    #[test]
    fn parser_invalid() {
        for input in [
            "rekt 3x2",
            "rect",
            "rect 34",
            "rotate",
            "rotate row",
            "rotate row y1 by 1",
            "rotate row y=1",
            "rotate row y=1 by",
            "rotate row x=1 by 2",
        ] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
    }

    #[test]
    fn solve_out_of_bounds() {
        let solution_data = InputData::from_str("rotate row y=150 by 2").unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("row 150 out of bounds".to_string()));

        let solution_data = InputData::from_str("rotate column x=150 by 2").unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("col 150 out of bounds".to_string()));
    }
}
