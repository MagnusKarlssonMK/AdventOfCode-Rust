//! # 2022 day 15 - Beacon Exclusion Zone
//!
//! Part 2 looks for the one position in the search area that no sensor covers. The solution
//! assumes that this position lies where the outer boundaries of two sensors' covered diamonds
//! cross, which is true for the puzzle inputs; this lets it test a small number of candidate
//! points instead of scanning the whole area.
//!
//! Each sensor covers a diamond whose four edges are lines of slope +1 or -1. The missing beacon
//! lies just outside the covered area, so it sits on one of those edges shifted one step outwards
//! (`range + 1`). Intersecting every rising edge with every falling edge gives the candidate set;
//! the answer is the first candidate inside the search square that no sensor covers. Only edges
//! shared by at least two sensors are kept, which shrinks the candidate set further.
use crate::aoc_util::{error::AocError, point::*};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const P1_ROW: i32 = 2_000_000;
const P2_MAX_COORD: i32 = 4_000_000;

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
    range: i32,
}

impl FromStr for Sensor {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(['=', ',', ':']).collect::<Vec<_>>().as_slice() {
            [
                "Sensor at x",
                x1,
                " y",
                y1,
                " closest beacon is at x",
                x2,
                " y",
                y2,
            ] => {
                let position = Point::new(x1.parse()?, y1.parse()?);
                let beacon = Point::new(x2.parse()?, y2.parse()?);
                let range = i32::try_from(position.manhattan(&beacon))
                    .map_err(|_| AocError::Invalid(s.to_string()))?;
                Ok(Self {
                    position,
                    beacon,
                    range,
                })
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Sensor {
    fn is_inrange(&self, pos: &Point) -> bool {
        self.range as usize >= self.position.manhattan(pos)
    }

    /// The four edges of the covered area grown by one step, as `(slope, intercept)`
    /// for `y = slope * x + intercept`.
    fn outer_edges(&self) -> [(i32, i32); 4] {
        let r = self.range + 1;
        let (x, y) = (self.position.x, self.position.y);
        [
            (1, y - r - x),
            (1, y + r - x),
            (-1, y - r + x),
            (-1, y + r + x),
        ]
    }
}

struct InputData {
    sensors: Vec<Sensor>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sensors: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn get_coverage(&self, row: i32) -> u64 {
        let mut x_ranges = Vec::new();
        for sensor in &self.sensors {
            let x_at_row = sensor.range - row.abs_diff(sensor.position.y) as i32;
            if x_at_row >= 0 {
                x_ranges.push((sensor.position.x - x_at_row, sensor.position.x + x_at_row));
            }
        }
        x_ranges.sort();
        // Merge overlapping ranges
        let mut merged: Vec<(i32, i32)> = Vec::new();
        for &(low, high) in &x_ranges {
            if let Some((_, merged_high)) = merged.last_mut()
                && low <= *merged_high
            {
                *merged_high = (*merged_high).max(high);
                continue;
            }
            merged.push((low, high));
        }

        let mut total: i64 = merged
            .iter()
            .map(|&(low, high)| 1 + i64::from(high) - i64::from(low))
            .sum();
        // Subtract 1 for every beacon inside those ranges
        let beacon_positions: HashSet<Point> = self
            .sensors
            .iter()
            .filter(|s| s.beacon.y == row)
            .map(|s| s.beacon)
            .collect();
        for beacon in &beacon_positions {
            if merged
                .iter()
                .any(|(low, high)| (*low..=*high).contains(&beacon.x))
            {
                total -= 1;
            }
        }
        total as u64
    }

    fn position_is_dark(&self, pos: &Point) -> bool {
        for sensor in &self.sensors {
            if sensor.is_inrange(pos) {
                return false;
            }
        }
        true
    }

    fn get_darkpointfreq(&self, maxsize: i32) -> Result<u64, AocError> {
        let mut lines = HashMap::new();
        for sensor in &self.sensors {
            for edge in sensor.outer_edges() {
                *lines.entry(edge).or_insert(0) += 1;
            }
        }
        // Filter out and keep only the lines that appear at least twice
        let mut ascending = Vec::new();
        let mut descending = Vec::new();
        for ((line1, line2), nbr) in &lines {
            if *nbr > 1 {
                if *line1 == 1 {
                    ascending.push(*line2);
                } else {
                    descending.push(*line2);
                }
            }
        }
        let mut positions = Vec::new();
        for asc_b in &ascending {
            for desc_b in &descending {
                let x = (desc_b - asc_b) / 2;
                positions.push(Point::new(x, x + asc_b));
            }
        }

        for p in &positions {
            if (0..=maxsize).contains(&p.x)
                && (0..=maxsize).contains(&p.y)
                && self.position_is_dark(p)
            {
                return Ok(p.x as u64 * P2_MAX_COORD as u64 + p.y as u64);
            }
        }
        Err(AocError::NoSolution)
    }

    fn solve_part1(&self) -> u64 {
        self.get_coverage(P1_ROW)
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        self.get_darkpointfreq(P2_MAX_COORD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    const TEST_CUSTOM_1: &str = "Sensor at x=0, y=2000000: closest beacon is at x=1, y=2000000
Sensor at x=0, y=2000001: closest beacon is at x=0, y=2000003
Sensor at x=3999999, y=2000001: closest beacon is at x=3999998, y=2000001";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.get_coverage(10), 26);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.get_darkpointfreq(20).unwrap(), 56000011);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
        assert_eq!(solution_data.solve_part2().unwrap(), 1999998);
    }
}
