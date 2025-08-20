//! # 2016 day 2 - Bathroom Security
use crate::aoc_util::{grid::*, point::*};
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((solution_data.solve_part1(), solution_data.solve_part2()))
}

impl Point {
    #[inline]
    fn to_dir(input: &char) -> Self {
        match input {
            'L' => LEFT,
            'R' => RIGHT,
            'U' => UP,
            'D' => DOWN,
            _ => ORIGIN,
        }
    }
}

struct InputData<'a> {
    instructions: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            instructions: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn get_bathroom_code(&self, keypad: &Grid) -> String {
        let mut result = Vec::new();
        let mut current = keypad.find('5').unwrap();
        for row in &self.instructions {
            for step in row.chars().map(|c| Point::to_dir(&c)) {
                let next = step + current;
                if let Some(e) = keypad.get_element(&next)
                    && e != ' '
                {
                    current = next;
                }
            }
            result.push(keypad.get_element(&current).unwrap());
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

    const TEST_DATA: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), "1985");
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), "5DB3");
    }
}
