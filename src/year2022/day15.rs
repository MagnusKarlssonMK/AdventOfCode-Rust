//! # 2022 day 15 - Beacon Exclusion Zone
//!
//! Transfered straight from python solution; probably room for improvement.
use std::collections::{HashMap, HashSet};

use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
    range: usize,
}

impl Sensor {
    fn new(input: &str) -> Self {
        let mut nbrs: Vec<i32> = Vec::new();
        for s in input.split(&['=', ',', ':'][..]) {
            if let Ok(n) = s.parse() {
                nbrs.push(n);
            }
        }
        let position = Point::new(nbrs[0], nbrs[1]);
        let beacon = Point::new(nbrs[2], nbrs[3]);
        let range = position.manhattan(&beacon);
        Self {
            position,
            beacon,
            range,
        }
    }

    fn is_inrange(&self, pos: &Point) -> bool {
        self.range >= self.position.manhattan(pos)
    }
}

struct InputData {
    sensors: Vec<Sensor>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            sensors: input.lines().map(Sensor::new).collect(),
        }
    }

    fn get_coverage(&self, row: usize) -> usize {
        let mut x_ranges = Vec::new();
        for sensor in &self.sensors {
            let x_at_row = sensor.range as i32 - (row as i32 - sensor.position.y).abs();
            if x_at_row > 0 {
                x_ranges.push((sensor.position.x - x_at_row, sensor.position.x + x_at_row));
            }
        }
        x_ranges.sort();
        // Merge overlapping ranges
        let mut merged_x_ranges = Vec::from([*x_ranges.first().unwrap()]);
        for (r_low, r_high) in x_ranges.iter().skip(1) {
            let (vx, vy) = merged_x_ranges.last_mut().unwrap();
            if (*vx..=*vy).contains(r_low) {
                if !(*vx..=*vy).contains(r_high) {
                    *vy = *r_high;
                }
            } else {
                merged_x_ranges.push((*r_low, *r_high));
            }
        }
        let mut total: i32 = merged_x_ranges
            .iter()
            .map(|(low, high)| 1 + high - low)
            .sum();
        // Subtract 1 for every beacon inside those ranges
        let beacon_positions: HashSet<Point> = self
            .sensors
            .iter()
            .filter(|s| s.beacon.y == row as i32)
            .map(|s| s.beacon)
            .collect();
        for beacon in beacon_positions.iter() {
            for (r1, r2) in &merged_x_ranges {
                if (*r1..=*r2).contains(&beacon.x) {
                    total -= 1;
                }
            }
        }
        total as usize
    }

    fn position_is_dark(&self, pos: &Point) -> bool {
        for sensor in &self.sensors {
            if sensor.is_inrange(pos) {
                return false;
            }
        }
        true
    }

    fn get_darkpointfreq(&self, maxsize: usize) -> usize {
        let mut lines = HashMap::new();
        for sensor in self.sensors.iter() {
            // Create 4 lines representing the outsides of the sensor's area, y = ax + b, a = [1, -1]
            // Tuple values (a, b)
            let range = sensor.range as i32;
            let upper_left = (1, sensor.position.y - range - 1 - sensor.position.x);
            let upper_right = (-1, sensor.position.y - range - 1 + sensor.position.x);
            let lower_left = (-1, sensor.position.y + range + 1 + sensor.position.x);
            let lower_right = (1, sensor.position.y + range + 1 - sensor.position.x);
            for line in &[upper_left, upper_right, lower_left, lower_right] {
                lines.entry(*line).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        //Filter out and keep only the lines that appear at least twice
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
            if (0..maxsize as i32).contains(&p.x)
                && (0..maxsize as i32).contains(&p.y)
                && self.position_is_dark(p)
            {
                return p.x as usize * 4_000_000 + p.y as usize;
            }
        }
        0
    }

    fn solve_part1(&self) -> usize {
        self.get_coverage(2_000_000)
    }

    fn solve_part2(&self) -> usize {
        self.get_darkpointfreq(4_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.get_coverage(10), 26);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.get_darkpointfreq(20), 56000011);
    }
}
