//! # 2019 day 3 - Crossed Wires
//!
//! ## Part 1
//!
//! We can find all intersections by putting all coordinates in sets for each wire, and then find the
//! common points. Then the answer is given by the smallest manhattan distance from origin to intersection.
//!
//! ## Part 2
//!
//! We can get the distances by finding the index of each intersection's coordinate in the wire lists.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    point::*,
};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

impl Point {
    fn dir_to_point(d: char) -> Result<Self, AocError> {
        match d {
            'R' => Ok(RIGHT),
            'L' => Ok(LEFT),
            'U' => Ok(UP),
            'D' => Ok(DOWN),
            _ => Err(AocError::Invalid(d.to_string())),
        }
    }
}

struct Wire {
    instructions: Vec<(Point, usize)>,
}

impl FromStr for Wire {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_instruction(s: &str) -> Result<(Point, usize), AocError> {
            let c = s.chars().next().ctx("instruction char")?;
            let p = Point::dir_to_point(c)?;
            let v = s.get(1..).ctx("instruction value")?.parse()?;
            Ok((p, v))
        }
        Ok(Self {
            instructions: s
                .split(',')
                .map(parse_instruction)
                .collect::<Result<_, _>>()?,
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
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('\n').ctx("line break")?;
        Ok(Self {
            wire_1: left.parse()?,
            wire_2: right.parse()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, usize), AocError> {
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
            let w1_steps = w1_points
                .iter()
                .position(|n| n == i)
                .expect("intersection must exist in w1");
            let w2_steps = w2_points
                .iter()
                .position(|n| n == i)
                .expect("intersection must exist in w2");
            p2.push(2 + w1_steps + w2_steps);
        }
        let p1 = p1.iter().min().ok_or(AocError::NoSolution)?;
        let p2 = p2.iter().min().ok_or(AocError::NoSolution)?;
        Ok((*p1, *p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "R8,U5,L5,D3
U7,R6,D4,L4";

    const TEST_OFFICIAL_2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    const TEST_OFFICIAL_3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, _) = solution_data.solve().unwrap();
        assert_eq!(p1, 6);
    }

    #[test]
    fn part1_2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 159);
        assert_eq!(p2, 610);
    }

    #[test]
    fn part1_2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 135);
        assert_eq!(p2, 410);
    }
}
