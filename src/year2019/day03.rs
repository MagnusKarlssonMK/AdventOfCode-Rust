//! # 2019 day 3 - Crossed Wires
use crate::aoc_util::point::*;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

impl Point {
    fn dir_to_point(d: char) -> Self {
        match d {
            'R' => RIGHT,
            'L' => LEFT,
            'U' => UP,
            'D' => DOWN,
            _ => unreachable!(),
        }
    }
}

struct Wire {
    instructions: Vec<(Point, usize)>,
}

impl FromStr for Wire {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .split(',')
                .map(|s| {
                    (
                        Point::dir_to_point(s.chars().nth(0).unwrap()),
                        s[1..].parse().unwrap(),
                    )
                })
                .collect(),
        })
    }
}

impl Wire {
    fn walk(&self) -> Vec<Point> {
        let mut pos = ORIGIN;
        let mut points = Vec::new();
        for (p, v) in &self.instructions {
            for _ in 0..*v {
                pos += *p;
                if pos != ORIGIN {
                    points.push(pos);
                }
            }
        }
        points
    }
}

struct InputData {
    wire_1: Wire,
    wire_2: Wire,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('\n').unwrap();
        Ok(Self {
            wire_1: Wire::from_str(left).unwrap(),
            wire_2: Wire::from_str(right).unwrap(),
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let w1_points = self.wire_1.walk();
        let w2_points = self.wire_2.walk();
        let mut w1_set: HashSet<Point> = HashSet::new();
        w1_set.extend(&w1_points);
        let mut w2_set: HashSet<Point> = HashSet::new();
        w2_set.extend(&w2_points);
        let intersections = w1_set.intersection(&w2_set);
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        for i in intersections {
            p1.push(i.manhattan(&ORIGIN));
            p2.push(
                2 + w1_points.iter().position(|n| n == i).unwrap()
                    + w2_points.iter().position(|n| n == i).unwrap(),
            );
        }
        (*p1.iter().min().unwrap(), *p2.iter().min().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 6);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 159);
        assert_eq!(p2, 610);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 135);
        assert_eq!(p2, 410);
    }
}
