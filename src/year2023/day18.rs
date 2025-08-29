//! # 2023 day 18 - Lavaduct Lagoon
use crate::aoc_util::point::{self, Point};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Instruction {
    direction: Point,
    steps: usize,
    color: String,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = match parts.next().unwrap() {
            "L" => point::LEFT,
            "R" => point::RIGHT,
            "U" => point::UP,
            "D" => point::DOWN,
            _ => unreachable!(),
        };
        let steps = parts.next().unwrap().parse().unwrap();
        let color = parts
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')')
            .to_string();
        Ok(Self {
            direction,
            steps,
            color,
        })
    }
}

impl Instruction {
    fn get_swapped_instruction(&self) -> Self {
        let direction = match self.color.chars().last().unwrap() {
            '0' => point::RIGHT,
            '1' => point::DOWN,
            '2' => point::LEFT,
            '3' => point::UP,
            _ => unreachable!(),
        };
        let steps = usize::from_str_radix(&self.color[0..5], 16).unwrap();
        Self {
            direction,
            steps,
            color: self.color.to_string(),
        }
    }
}

impl Point {
    #[inline]
    fn get_determinant(&self, other: &Self) -> i64 {
        // Need to use 64 bit numbers since the coordinates get really large for part 2
        (self.x as i64 * other.y as i64) - (self.y as i64 * other.x as i64)
    }
}

struct InputData {
    plan: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            plan: s
                .lines()
                .map(|line| Instruction::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn get_area_points(&self, swapped: bool) -> usize {
        let mut digpath = vec![point::ORIGIN];
        for instr in &self.plan {
            let next = if !swapped {
                *digpath.last().unwrap() + (instr.direction) * (instr.steps as i32)
            } else {
                let swapped = instr.get_swapped_instruction();
                *digpath.last().unwrap() + (swapped.direction) * (swapped.steps as i32)
            };
            digpath.push(next);
        }
        // Add starting point at the end to close the loop so we can use the windows function
        digpath.push(point::ORIGIN);

        let (areasum, outline_len) =
            digpath
                .windows(2)
                .fold((0, 0), |(areasum, outline_len), p| {
                    (
                        areasum + p[0].get_determinant(&p[1]),
                        outline_len + p[0].manhattan(&p[1]),
                    )
                });
        let areasum = areasum.unsigned_abs() as usize;
        (areasum / 2) + 1 + (outline_len / 2)
    }

    fn solve_part1(&self) -> usize {
        self.get_area_points(false)
    }

    fn solve_part2(&self) -> usize {
        self.get_area_points(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 62);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 952408144115);
    }
}
