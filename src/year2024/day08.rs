//! # 2024 day 8 - Resonant Collinearity
//! 
//! Solves both part 1 and part 2 in one go. For all points belonging to
//! the same frequency (grid character), loop through the pair combinations
//! and generate their antinode-points until both points are outside the grid.
//! Add those antinode-points to hashsets and determine the answers to the
//! respective part from their final lengths. Remember that also the original
//! nodes needs to be included for the part2 calculation.
use crate::aoc_util::point::*;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

impl Point {
    fn get_anti_points(&self, other: &Point, harmonic: i32) -> (Point, Point) {
        (
            Point::new(
                self.x + harmonic * (self.x - other.x),
                self.y + harmonic * (self.y - other.y),
            ),
            Point::new(
                other.x + harmonic * (other.x - self.x),
                other.y + harmonic * (other.y - self.y),
            ),
        )
    }
}

struct InputData {
    antennas: HashMap<char, Vec<Point>>,
    map_width: usize,
    map_height: usize,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut antennas = HashMap::new();
        let mut map_height = 0;
        let mut map_width = 0;
        for (y, line) in input.lines().enumerate() {
            map_height += 1;
            if y == 0 {
                map_width = line.len();
            }
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push(Point::new(x as i32, y as i32));
                }
            }
        }
        Self {
            antennas,
            map_width,
            map_height,
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut antinodes = HashSet::new();
        let mut antinodes_w_harmonics = HashSet::new();
        for points in self.antennas.values() {
            for (i, p1) in points.iter().enumerate() {
                for p2 in points.iter().skip(i + 1) {
                    let mut inside = true;
                    let mut harmonic = 0;
                    while inside {
                        inside = false;
                        let (an1, an2) = p1.get_anti_points(p2, harmonic);
                        if (0..self.map_width as i32).contains(&an1.x)
                            && (0..self.map_height as i32).contains(&an1.y)
                        {
                            inside = true;
                            if harmonic == 1 {
                                antinodes.insert(an1);
                            }
                            antinodes_w_harmonics.insert(an1);
                        }
                        if (0..self.map_width as i32).contains(&an2.x)
                            && (0..self.map_height as i32).contains(&an2.y)
                        {
                            inside = true;
                            if harmonic == 1 {
                                antinodes.insert(an2);
                            }
                            antinodes_w_harmonics.insert(an2);
                        }
                        harmonic += 1;
                    }
                }
            }
        }
        (antinodes.len(), antinodes_w_harmonics.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 14);
        assert_eq!(p2, 34);
    }
}
