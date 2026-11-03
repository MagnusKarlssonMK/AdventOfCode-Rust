//! # 2018 day 10 - The Stars Align
//!
//! The message will be visible at the step when the covered area is the smallest. So
//! start from the starting state, keep taking steps until the area would increase from
//! taking another step. The answer for part 1 is then given by rendering the image
//! from the current positions of the lights, and for part 2 the number of steps taken
//! to get there.
use crate::aoc_util::{error::*, grid::Grid, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1, p2.to_string()))
}

#[derive(Debug, Clone, Copy)]
struct Position {
    point: Point,
    velocity: Point,
}

impl FromStr for Position {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(['<', '>']).collect::<Vec<_>>().as_slice() {
            ["position=", p, " velocity=", v, _] => Ok(Self {
                point: p.parse()?,
                velocity: v.parse()?,
            }),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Position {
    #[inline]
    fn step(&self) -> Self {
        Self {
            point: self.point + self.velocity,
            velocity: self.velocity,
        }
    }
}

/// Returns one point with the smallest x and y values, and one point with the largest
/// x and y values, taken from the points of each position. Velocities are not used.
/// Returns `None` if `p` is empty.
#[inline]
fn get_bounds(p: &[Position]) -> Option<(Point, Point)> {
    let xs: Vec<_> = p.iter().map(|p| p.point.x).collect();
    let ys: Vec<_> = p.iter().map(|p| p.point.y).collect();
    let xmin = xs.iter().min()?;
    let xmax = xs.iter().max()?;
    let ymin = ys.iter().min()?;
    let ymax = ys.iter().max()?;
    Some((Point::new(*xmin, *ymin), Point::new(*xmax, *ymax)))
}

/// Calculates the area of the box spanned by the points of a set of positions. Returns
/// `None` if `p`is empty.
#[inline]
fn get_box_area(p: &[Position]) -> Option<usize> {
    let (bmin, bmax) = get_bounds(p)?;
    // Note: +1 for each axis since the bmax point should be inside the box
    Some((1 + bmax.x.abs_diff(bmin.x) as usize) * (1 + bmax.y.abs_diff(bmin.y) as usize))
}

/// Generates the message from the current light positions `p`. Returns `None` if
/// `p`is empty.
fn render_grid(p: &[Position]) -> Option<String> {
    let (bmin, bmax) = get_bounds(p)?;
    let mut grid = Grid::new(
        1 + bmax.x.abs_diff(bmin.x) as usize,
        1 + bmax.y.abs_diff(bmin.y) as usize,
        '.',
    );
    for light in p.iter().map(|pos| pos.point - bmin) {
        grid.set_point(&light, '#');
    }
    Some(format!("\n{}", grid))
}

#[derive(Debug)]
struct InputData {
    positions: Vec<Position>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            positions: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> (String, usize) {
        let mut positions = self.positions.clone();
        let mut area =
            get_box_area(&self.positions).expect("positions should always have > 0 positions");
        let mut seconds = 0;
        let mut new_positions: Vec<Position>;
        loop {
            new_positions = positions.iter().map(|p| p.step()).collect();
            let new_area = get_box_area(&new_positions)
                .expect("new_positions should always have > 0 positions");
            if new_area < area {
                area = new_area;
                positions = new_positions.clone();
                seconds += 1;
            } else {
                let message = if area < 1000 {
                    render_grid(&positions).expect("cannot render empty positions")
                } else {
                    // Safety guard against unexpected input so we don't try to build an enormous grid
                    // Real input should end up with an area of 620
                    "too large".to_string()
                };
                return (message, seconds);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    const EXPECTED_MESSAGE: &str = "
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, EXPECTED_MESSAGE);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 3);
    }

    #[test]
    fn test_position() {
        let test_str = "position=<-6,  0> velocity=< 2,  -1>";
        let pos: Position = test_str.parse().unwrap();
        assert_eq!(pos.point, Point { x: -6, y: 0 });
        assert_eq!(pos.velocity, Point { x: 2, y: -1 });
        assert_eq!(pos.step().point, Point { x: -4, y: -1 });

        let test_str = "position=<-6,  0> velocity=< 2,  -1>
position=<4,  3> velocity=< -4,  5>
position=<-2,  -1> velocity=< 0,  6>";
        let positions: Vec<Position> = test_str.lines().map(|line| line.parse().unwrap()).collect();
        let (a, b) = get_bounds(&positions).unwrap();
        assert_eq!(a, Point { x: -6, y: -1 });
        assert_eq!(b, Point { x: 4, y: 3 });
        let area = get_box_area(&positions).unwrap();
        assert_eq!(area, 55);
    }
}
