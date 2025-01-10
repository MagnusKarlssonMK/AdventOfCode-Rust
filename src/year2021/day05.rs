//! # 2021 day 5 - Hydrothermal Venture
use crate::aoc_util::{grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

impl Point {
    fn is_diagonal(&self, other: &Point) -> bool {
        self.x != other.x && self.y != other.y
    }

    fn get_derivate(&self, other: &Point) -> Point {
        let dx = (other.x - self.x)
            / (other.x - self.x)
                .abs()
                .max((other.y - self.y).abs())
                .max(1);
        let dy = (other.y - self.y)
            / (other.x - self.x)
                .abs()
                .max((other.y - self.y).abs())
                .max(1);
        Point::new(dx, dy)
    }
}

struct OceanFloor {
    points: Grid,
    nbr_dangerous_points: usize,
}

impl OceanFloor {
    fn new(x: usize, y: usize) -> Self {
        Self {
            points: Grid::new(x, y, '0'),
            nbr_dangerous_points: 0,
        }
    }

    fn process_line(&mut self, p1: &Point, p2: &Point) {
        let dxdy = p1.get_derivate(p2);
        let mut p = *p1;
        loop {
            match self.points.get_element(&p) {
                Some('0') => self.points.set_point(&p, '1'),
                Some('1') => {
                    self.nbr_dangerous_points += 1;
                    self.points.set_point(&p, '2');
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_max = 0;
        let mut y_max = 0;
        let mut straight_lines = Vec::new();
        let mut diagonal_lines = Vec::new();
        s.lines().for_each(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            let p1 = Point::from_str(p1).unwrap();
            let p2 = Point::from_str(p2).unwrap();
            if p1.is_diagonal(&p2) {
                diagonal_lines.push((p1, p2));
            } else {
                straight_lines.push((p1, p2));
            }
            x_max = x_max.max(p1.x as usize).max(p2.x as usize);
            y_max = y_max.max(p1.y as usize).max(p2.y as usize);
        });
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

    #[test]
    fn part1_example_1() {
        let testdata = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 5);
        assert_eq!(p2, 12);
    }
}
