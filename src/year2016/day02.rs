//! # 2016 day 2 - Bathroom Security
use crate::aoc_util::{error::AocError, grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((solution_data.solve_part1(), solution_data.solve_part2()))
}

impl Point {
    #[inline]
    fn to_dir(input: char) -> Result<Self, AocError> {
        match input {
            'L' => Ok(LEFT),
            'R' => Ok(RIGHT),
            'U' => Ok(UP),
            'D' => Ok(DOWN),
            _ => Err(AocError::Invalid(input.to_string())),
        }
    }
}

#[derive(Debug)]
struct InputData {
    instructions: Vec<Vec<Point>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .lines()
                .map(|line| line.chars().map(|c| Point::to_dir(c)).collect())
                .collect::<Result<Vec<Vec<_>>, _>>()?,
        })
    }
}

impl InputData {
    /// Takes a given `keypad` layout and generates the bathroom code from the instructions.
    /// Panics if `keypad` doesn't contain a `5`.
    fn get_bathroom_code(&self, keypad: &Grid) -> String {
        let mut result = Vec::new();
        let mut button = '5';
        let mut point = keypad.find(button).unwrap();
        for row in &self.instructions {
            for step in row {
                let next = *step + point;
                if let Some(e) = keypad.get_element(&next)
                    && e != ' '
                {
                    point = next;
                    button = e;
                }
            }
            result.push(button);
        }
        result.iter().collect()
    }

    fn solve_part1(&self) -> String {
        let keypad = Grid::parse("123\n456\n789");
        self.get_bathroom_code(&keypad)
    }

    fn solve_part2(&self) -> String {
        let keypad = Grid::parse("  1  \n 234 \n56789\n ABC \n  D  ");
        self.get_bathroom_code(&keypad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "ULL
RRDDD
LURDL
UUUUD";

    const TEST_CUSTOM_1: &str = "ULD
UR
D
UL
RR
D
UL
LDD
UR
LU
RDD
UL
RR
LU
RDD
UL
RU
LLDDR
LU
RDL
RR
LU
RDL
RU";

    const TEST_CUSTOM_2: &str = "RURUD
LD
UR
D
UL
RR
LU
DRD
UL
LDLR
D
UL
RR
LU
RDD
UL
RR
LU
RDD
UL
RR
LU
DRL
LLDR
LU
RDD
UL
RR
LU
RDL
RU
LDDU";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), "1985");
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), "425136275184629538479586");
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), "5DB3");
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(
            solution_data.solve_part2(),
            "3637241836A572B683C7948B6DAC7B8B"
        );
    }

    #[test]
    fn parser_invalid_input() {
        let err = InputData::from_str("RLUJD").unwrap_err();
        assert_eq!(err, AocError::Invalid("J".to_string()));
    }
}
