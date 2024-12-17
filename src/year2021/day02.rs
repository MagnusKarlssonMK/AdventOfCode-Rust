//! # 2021 day 2 - Dive!
//!
//! Some practice with rust enums.
use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl Command {
    fn from(s: &str) -> Self {
        let (_, right) = s.split_once(' ').unwrap();
        let val: usize = right.parse().unwrap();
        match s.chars().nth(0).unwrap() {
            'u' => Self::Up(val),
            'd' => Self::Down(val),
            'f' => Self::Forward(val),
            _ => unreachable!(),
        }
    }

    fn get_move(&self) -> Point {
        match self {
            Self::Up(v) => Point::new(0, -(*v as i32)),
            Self::Down(v) => Point::new(0, *v as i32),
            Self::Forward(v) => Point::new(*v as i32, 0),
        }
    }

    fn get_aimed_move(&self, current_aim: i32) -> MoveResult {
        match self {
            Self::Up(v) => MoveResult::Aim(-(*v as i32)),
            Self::Down(v) => MoveResult::Aim(*v as i32),
            Self::Forward(v) => {
                MoveResult::Position(Point::new(*v as i32, *v as i32 * current_aim))
            }
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

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            commands: input.lines().map(Command::from).collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        let mut position = ORIGIN;
        self.commands
            .iter()
            .for_each(|cmd| position += cmd.get_move());
        (position.x * position.y) as usize
    }

    fn solve_part2(&self) -> usize {
        let mut position = ORIGIN;
        let mut aim = 0;
        self.commands.iter().for_each(|cmd| {
            match cmd.get_aimed_move(aim) {
                MoveResult::Aim(v) => aim += v,
                MoveResult::Position(v) => position += v,
            };
        });
        (position.x * position.y) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 150);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 900);
    }
}
