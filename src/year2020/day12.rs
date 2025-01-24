//! # 2020 day 12 - Rain Risk
use crate::aoc_util::point::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
enum Instruction {
    MoveDir(Point, usize),
    TurnLeft(usize),
    TurnRight(usize),
    MoveForward(usize),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse().unwrap();
        match s.chars().nth(0).unwrap() {
            'N' => Ok(Self::MoveDir(UP, value)),
            'S' => Ok(Self::MoveDir(DOWN, value)),
            'E' => Ok(Self::MoveDir(RIGHT, value)),
            'W' => Ok(Self::MoveDir(LEFT, value)),
            'L' => Ok(Self::TurnLeft(value)),
            'R' => Ok(Self::TurnRight(value)),
            'F' => Ok(Self::MoveForward(value)),
            _ => Err(()),
        }
    }
}

struct InputData {
    instructions: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .lines()
                .map(|line| Instruction::from_str(line).unwrap())
                .collect(),
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
                        position += *d * (*v as i32);
                    } else {
                        direction += *d * (*v as i32);
                    }
                }
                Instruction::TurnLeft(v) => {
                    for _ in 0..v / 90 {
                        direction = direction.rotate_left();
                    }
                }
                Instruction::TurnRight(v) => {
                    for _ in 0..v / 90 {
                        direction = direction.rotate_right();
                    }
                }
                Instruction::MoveForward(v) => {
                    position += direction * (*v as i32);
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

    const TEST_DATA: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 25);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 286);
    }
}
