//! # 2020 day 12 - Rain Risk
//!
//! Navigate the ship, tracking either its facing direction (part 1) or a waypoint relative to it
//! (part 2). Rotations spin whichever of the two is tracked, and forward movement advances the
//! ship by it; the manhattan distance from the origin is then the answer.
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

#[derive(Debug)]
enum Instruction {
    MoveDir(Point, i32),
    TurnLeft(i32),
    TurnRight(i32),
    MoveForward(i32),
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s.chars().next().ctx("instruction char")?;
        let value = s.get(1..).ctx("value")?.parse()?;
        if value < 0 {
            return Err(AocError::Invalid(s.to_string()));
        }
        match instr {
            'N' => Ok(Self::MoveDir(UP, value)),
            'S' => Ok(Self::MoveDir(DOWN, value)),
            'E' => Ok(Self::MoveDir(RIGHT, value)),
            'W' => Ok(Self::MoveDir(LEFT, value)),
            'L' => Ok(Self::TurnLeft(value)),
            'R' => Ok(Self::TurnRight(value)),
            'F' => Ok(Self::MoveForward(value)),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

struct InputData {
    instructions: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn get_travel_distance(&self, use_waypoint: bool) -> usize {
        let mut position = ORIGIN;
        let mut direction = if use_waypoint {
            Point::new(10, -1)
        } else {
            RIGHT
        };
        for i in &self.instructions {
            match i {
                Instruction::MoveDir(d, v) => {
                    if !use_waypoint {
                        position += *d * *v;
                    } else {
                        direction += *d * *v;
                    }
                }
                Instruction::TurnLeft(v) => {
                    for _ in 0..(v / 90) % 4 {
                        direction = direction.rotate_left();
                    }
                }
                Instruction::TurnRight(v) => {
                    for _ in 0..(v / 90) % 4 {
                        direction = direction.rotate_right();
                    }
                }
                Instruction::MoveForward(v) => {
                    position += direction * *v;
                }
            }
        }
        position.manhattan(&ORIGIN)
    }

    fn solve_part1(&self) -> usize {
        self.get_travel_distance(false)
    }

    fn solve_part2(&self) -> usize {
        self.get_travel_distance(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "F10
N3
F7
R90
F11";

    /// Turns left and then moves, exercising the rotate_left path.
    const TEST_CUSTOM_1: &str = "L90
F1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 25);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 286);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
        assert_eq!(solution_data.solve_part2(), 11);
    }
}
