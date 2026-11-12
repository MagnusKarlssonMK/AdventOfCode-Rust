//! # 2021 day 5 - Hydrothermal Venture
//!
//! Split the input into straight and diagonal line segments, tracking the largest coordinate on
//! each axis so that the ocean floor can be sized to fit. Each point on the floor is promoted from
//! clear to covered the first time a line reaches it, and from covered to dangerous the first time
//! a second line reaches it, so counting promotions counts every dangerous point exactly once no
//! matter how many lines happen to cross there.
//!
//! Both parts can then be solved in one go; first apply only the straight lines to get the answer
//! for Part 1, and then apply the diagonal lines on top of that to get the answer for Part 2.
//!
//! Lines must be horizontal, vertical, or exactly 45 degrees diagonal, and all coordinates must be
//! non-negative; anything else is rejected as invalid input.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    grid::*,
    point::*,
};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

enum LineType {
    /// Horizontal or vertical
    Straight,
    /// Exactly 45 degrees diagonal
    Diagonal,
    /// Neither horizontal nor diagonal, or both points are the same
    Invalid,
}

impl Point {
    /// Returns the line type generated between two points
    fn get_line_type(&self, other: &Point) -> LineType {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if dx == 0 && dy == 0 {
            LineType::Invalid
        } else if dx == 0 || dy == 0 {
            LineType::Straight
        } else if dx.abs() == dy.abs() {
            LineType::Diagonal
        } else {
            LineType::Invalid
        }
    }

    fn get_derivate(&self, other: &Point) -> Point {
        Point::new((other.x - self.x).signum(), (other.y - self.y).signum())
    }
}

struct OceanFloor {
    /// Grid of number of overlapping lines at each point; 0 = safe, 1 = covered, 2 = dangerous.
    points: Grid<u8>,
    nbr_dangerous_points: usize,
}

impl OceanFloor {
    fn new(x: usize, y: usize) -> Self {
        Self {
            points: Grid::new(x, y, &0),
            nbr_dangerous_points: 0,
        }
    }

    fn process_line(&mut self, p1: &Point, p2: &Point) {
        let dxdy = p1.get_derivate(p2);
        let mut p = *p1;
        loop {
            match self.points.get_element(&p) {
                Some(0) => self.points.set_point(&p, 1),
                Some(1) => {
                    self.nbr_dangerous_points += 1;
                    self.points.set_point(&p, 2);
                }
                _ => (),
            };
            if p == *p2 {
                break;
            }
            p += dxdy;
        }
    }
}

struct InputData {
    straight_lines: Vec<(Point, Point)>,
    diagonal_lines: Vec<(Point, Point)>,
    x_max: usize,
    y_max: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_max = 0;
        let mut y_max = 0;
        let mut straight_lines = Vec::new();
        let mut diagonal_lines = Vec::new();
        for line in s.lines() {
            let (p1, p2) = line
                .split_once(" -> ")
                .ctx(format!("missing separator: {line:?}"))?;
            let p1: Point = p1.parse()?;
            let p2: Point = p2.parse()?;
            if p1.x < 0 || p1.y < 0 || p2.x < 0 || p2.y < 0 {
                return Err(AocError::Invalid(format!(
                    "negative coordinate in {line:?}"
                )));
            }
            match p1.get_line_type(&p2) {
                LineType::Straight => straight_lines.push((p1, p2)),
                LineType::Diagonal => diagonal_lines.push((p1, p2)),
                LineType::Invalid => {
                    return Err(AocError::Invalid(format!("invalid line in {line:?}")));
                }
            }
            x_max = x_max.max(p1.x as usize).max(p2.x as usize);
            y_max = y_max.max(p1.y as usize).max(p2.y as usize);
        }
        Ok(Self {
            straight_lines,
            diagonal_lines,
            x_max,
            y_max,
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut ocean_floor = OceanFloor::new(self.x_max + 1, self.y_max + 1);
        self.straight_lines
            .iter()
            .for_each(|(p1, p2)| ocean_floor.process_line(p1, p2));
        let part1 = ocean_floor.nbr_dangerous_points;

        self.diagonal_lines
            .iter()
            .for_each(|(p1, p2)| ocean_floor.process_line(p1, p2));
        (part1, ocean_floor.nbr_dangerous_points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 5);
        assert_eq!(p2, 12);
    }

    #[test]
    fn parser_invalid_input() {
        assert!(matches!(
            "0,0 -> 2,1".parse::<InputData>(),
            Err(AocError::Invalid(_))
        )); // not 45°
        assert!(matches!(
            "-1,0 -> 0,0".parse::<InputData>(),
            Err(AocError::Invalid(_))
        )); // negative
        assert!(matches!(
            "1,1 -> 1,1".parse::<InputData>(),
            Err(AocError::Invalid(_))
        )); // same points
    }
}
