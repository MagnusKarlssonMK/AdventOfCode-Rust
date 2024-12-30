//! # 2022 day 9 - Rope Bridge
use crate::aoc_util::point::*;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
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

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            motions: input
                .lines()
                .map(|line| {
                    let (dir, len) = line.split_once(' ').unwrap();
                    (Point::parse_dir(dir), len.parse().unwrap())
                })
                .collect(),
        }
    }

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

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 36);
    }
}
