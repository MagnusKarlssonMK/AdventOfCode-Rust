//! # 2018 day 6 - Chronal Coordinates
//!
//! Key point here is that the min and max values in each direction forms a "frame", and any location outside
//! that will extend to infinity. I.e. any coordinate having a location as 'closest' on or outside that area
//! will have an infinite area.
//!
//! ## Part 1
//!
//! Just tracks, per cell, the nearest point (first/second smallest manhattan distance) and the area sizes.
//!
//! ## Part 2
//!
//! Uses the fact that the manhattan distance sum separates into an x-part and a y-part, each computed in
//! O(1) per coordinate with sorted prefix sums.
use crate::aoc_util::{error::*, point::Point};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const P2_MAX_TOTAL_DISTANCE: i32 = 10_000;

struct InputData {
    points: Vec<Point>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            points: s
                .lines()
                .map(|line| {
                    line.parse()
                        .map_err(|_| AocError::Invalid(line.to_string()))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let x_min = self.points.iter().map(|p| p.x).min().unwrap_or(0);
        let x_max = self.points.iter().map(|p| p.x).max().unwrap_or(0);
        let y_min = self.points.iter().map(|p| p.y).min().unwrap_or(0);
        let y_max = self.points.iter().map(|p| p.y).max().unwrap_or(0);
        let mut counts = HashMap::new(); // point index -> closest cells
        let mut border = HashSet::new(); // point indices touching the border
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let mut first = usize::MAX; // Smallest distance
                let mut second = usize::MAX; // Second smallest distance
                let mut first_i = 0; // Index of first smallest distance
                let p = Point::new(x, y);
                for (i, c) in self.points.iter().enumerate() {
                    let d = p.manhattan(c);
                    if d < first {
                        second = first;
                        first = d;
                        first_i = i;
                    } else if d < second {
                        second = d;
                    }
                }
                if first < second {
                    // Skip cells that are equally close to two points
                    counts
                        .entry(first_i)
                        .and_modify(|v| *v += 1)
                        .or_insert(1usize);
                    if ([x_min, x_max]).contains(&x) || ([y_min, y_max]).contains(&y) {
                        border.insert(first_i);
                    }
                }
            }
        }
        counts
            .iter()
            .filter(|(i, _)| !border.contains(*i))
            .map(|(_, v)| *v)
            .max()
            .unwrap_or(0)
    }

    fn solve_part2(&self) -> usize {
        let n = self.points.len();
        let mut xs: Vec<_> = self.points.iter().map(|p| p.x).collect();
        let mut ys: Vec<_> = self.points.iter().map(|p| p.y).collect();
        xs.sort_unstable();
        ys.sort_unstable();
        let mut prefix_x = vec![0; n + 1];
        let mut prefix_y = vec![0; n + 1];
        for i in 0..n {
            prefix_x[i + 1] = prefix_x[i] + xs[i];
            prefix_y[i + 1] = prefix_y[i] + ys[i];
        }
        let total_x = prefix_x[n];
        let total_y = prefix_y[n];
        let mut count = 0;
        for x in xs[0]..=*xs.last().unwrap() {
            let k1 = xs.partition_point(|v| *v <= x);
            let fx = x * (2 * k1 as i32 - n as i32) + total_x - 2 * prefix_x[k1];
            for y in ys[0]..=*ys.last().unwrap() {
                let k2 = ys.partition_point(|v| *v <= y);
                if fx + y * (2 * k2 as i32 - n as i32) + total_y - 2 * prefix_y[k2]
                    < P2_MAX_TOTAL_DISTANCE
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 17);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 72);
    }
}
