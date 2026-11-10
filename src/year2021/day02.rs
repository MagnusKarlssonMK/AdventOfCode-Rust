//! # 2021 day 2 - Dive!
//!
//! The submarine starts at the origin and a list of instructions tells it how to move. In part 1
//! the vertical moves directly change the depth, and a forward move only changes the horizontal
//! position. In part 2 the vertical moves instead adjust an aim value, and each forward move then
//! travels both horizontally and vertically by the current aim. Both parts end by multiplying
//! horizontal and vertical position.
//!
//! Small note: the puzzle description doesn't mention how to handle the hypothetical case of `y`
//! becoming negative, i.e. moving above the starting depth. The solution allows it to happen in
//! case an input would cause it to.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    point::*,
};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Command {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').ctx_invalid(format!("command {s:?}"))?;
        let val = right.parse()?;
        match left {
            "up" => Ok(Self::Up(val)),
            "down" => Ok(Self::Down(val)),
            "forward" => Ok(Self::Forward(val)),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Command {
    fn get_move(&self) -> Point {
        match self {
            Self::Up(v) => Point::new(0, -(*v)),
            Self::Down(v) => Point::new(0, *v),
            Self::Forward(v) => Point::new(*v, 0),
        }
    }

    fn get_aimed_move(&self, current_aim: i32) -> MoveResult {
        match self {
            Self::Up(v) => MoveResult::Aim(-(*v)),
            Self::Down(v) => MoveResult::Aim(*v),
            Self::Forward(v) => MoveResult::Position(Point::new(*v, *v * current_aim)),
        }
    }
}

enum MoveResult {
    Position(Point),
    Aim(i32),
}

struct InputData {
    commands: Vec<Command>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            commands: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> i64 {
        let mut position = ORIGIN;
        self.commands
            .iter()
            .for_each(|cmd| position += cmd.get_move());
        i64::from(position.x) * i64::from(position.y)
    }

    fn solve_part2(&self) -> i64 {
        let mut position = ORIGIN;
        let mut aim = 0;
        self.commands.iter().for_each(|cmd| {
            match cmd.get_aimed_move(aim) {
                MoveResult::Aim(v) => aim += v,
                MoveResult::Position(v) => position += v,
            };
        });
        i64::from(position.x) * i64::from(position.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 150);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 900);
    }

    #[test]
    fn parsing_invalid_input() {
        assert!("sideways 1".parse::<Command>().is_err());
        assert!("up2".parse::<Command>().is_err());
        assert!("up two".parse::<Command>().is_err());
    }
}
