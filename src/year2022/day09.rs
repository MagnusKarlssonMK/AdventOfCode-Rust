//! # 2022 day 9 - Rope Bridge
use crate::aoc_util::point::*;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

impl Point {
    fn parse_dir(input: &str) -> Self {
        match input {
            "R" => RIGHT,
            "L" => LEFT,
            "U" => UP,
            "D" => DOWN,
            _ => unreachable!(),
        }
    }

    fn catchup(&self, other: &Point) -> Self {
        Self {
            x: self.x + other.x.signum(),
            y: self.y + other.y.signum(),
        }
    }
}

struct InputData {
    motions: Vec<(Point, usize)>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            motions: s
                .lines()
                .map(|line| {
                    let (dir, len) = line.split_once(' ').unwrap();
                    (Point::parse_dir(dir), len.parse().unwrap())
                })
                .collect(),
        })
    }
}

impl InputData {
    fn get_nbr_tail_positions(&self, nbr_knots: usize) -> usize {
        let mut knotpos = vec![ORIGIN; nbr_knots];
        let mut tail_seen = HashSet::from([ORIGIN]);
        for &(dir, steps) in self.motions.iter() {
            for _ in 0..steps {
                knotpos[0] += dir;
                for i in 1..nbr_knots {
                    let diff = knotpos[i - 1] - knotpos[i];
                    if diff.x.abs() > 1 || diff.y.abs() > 1 {
                        knotpos[i] = knotpos[i].catchup(&diff);
                    }
                }
                tail_seen.insert(*knotpos.last().unwrap());
            }
        }
        tail_seen.len()
    }

    fn solve_part1(&self) -> usize {
        self.get_nbr_tail_positions(2)
    }

    fn solve_part2(&self) -> usize {
        self.get_nbr_tail_positions(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const TEST_DATA_2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 36);
    }
}
