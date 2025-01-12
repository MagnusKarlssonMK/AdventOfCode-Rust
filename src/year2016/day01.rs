//! # 2016 day 1 - No Time for a Taxicab
use crate::aoc_util::point::*;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

enum Rotation {
    Left,
    Right,
}

impl FromStr for Rotation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('L') => Ok(Self::Left),
            Some('R') => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

struct InputData {
    instructions: Vec<(Rotation, i32)>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .split(", ")
                .map(|word| {
                    let (left, right) = word.split_at(1);
                    (Rotation::from_str(left).unwrap(), right.parse().unwrap())
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut pos = ORIGIN;
        let mut direction = UP;
        for (r, i) in &self.instructions {
            direction = match r {
                Rotation::Left => direction.rotate_left(),
                Rotation::Right => direction.rotate_right(),
            };
            pos += direction * *i;
        }
        pos.manhattan(&ORIGIN)
    }

    fn solve_part2(&self) -> usize {
        let mut pos = ORIGIN;
        let mut direction = UP;
        let mut seen: HashSet<Point> = HashSet::new();
        seen.insert(pos);
        for (r, i) in &self.instructions {
            direction = match r {
                Rotation::Left => direction.rotate_left(),
                Rotation::Right => direction.rotate_right(),
            };
            for _ in 0..*i {
                pos += direction;
                if seen.contains(&pos) {
                    return pos.manhattan(&ORIGIN);
                }
                seen.insert(pos);
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = "R2, L3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "R2, R2, R2";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "R5, L5, R5, R3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "R8, R4, R4, R8";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
