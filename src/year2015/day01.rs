//! # 2015 day 1 - Not Quite Lisp
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    directions: Vec<isize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            directions: s
                .chars()
                .map(|c| match &c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> isize {
        self.directions.iter().sum()
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let mut floor = 0;
        self.directions
            .iter()
            .enumerate()
            .find(|(_, v)| {
                floor += *v;
                floor < 0
            })
            .map_or(Err(AocError::NoSolution), |(i, _)| Ok(i + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_official_1() {
        let solution_data: InputData = "(())".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = "()()".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = "(((".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_official_4() {
        let solution_data: InputData = "(()(()(".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_official_5() {
        let solution_data: InputData = "))(((((".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_official_6() {
        let solution_data: InputData = "())".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), -1);
    }

    #[test]
    fn part1_official_7() {
        let solution_data: InputData = "))(".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), -1);
    }

    #[test]
    fn part1_official_8() {
        let solution_data: InputData = ")))".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), -3);
    }

    #[test]
    fn part1_official_9() {
        let solution_data: InputData = ")())())".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), -3);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = ")".parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 1);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = "()())".parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 5);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = "(())()()((((()(()())(((((())))())))())())".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = "))(".parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 1);
    }
}
