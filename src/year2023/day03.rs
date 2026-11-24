//! # 2023 day 3 - Gear Ratios
//!
//! Numbers adjacent to a symbol, including diagonally, are part numbers; part 1 sums them.
//! For part 2, a `*` adjacent to exactly two part numbers is a gear, and its ratio is the
//! product of those two numbers.
use crate::aoc_util::{error::AocError, point::*};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct PartItem {
    point: Point,
    length: i32,
    value: u64,
}

impl PartItem {
    fn adjacent_points(&self) -> impl Iterator<Item = Point> {
        let start_x = self.point.x - 1;
        let end_x = self.point.x + self.length;
        let start_y = self.point.y - 1;
        let end_y = self.point.y + 1;
        (start_y..=end_y).flat_map(move |y| (start_x..=end_x).map(move |x| Point::new(x, y)))
    }
}

struct InputData {
    parts: HashMap<PartItem, HashSet<Point>>,
    symbols: HashMap<Point, char>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number: u64 = 0;
        let mut number_len = 0;
        let mut numberpoint = Point::new(0, 0);
        let mut parts = HashSet::new();
        let mut symbols = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    if number_len == 0 {
                        numberpoint = Point::new(x as i32, y as i32);
                    }
                    number = number
                        .checked_mul(10)
                        .and_then(|n| n.checked_add(digit as u64))
                        .ok_or_else(|| AocError::Overflow("part number overflow".to_string()))?;
                    number_len += 1;
                } else {
                    if number_len > 0 {
                        parts.insert(PartItem {
                            point: numberpoint,
                            length: number_len,
                            value: number,
                        });
                        number = 0;
                        number_len = 0;
                    }
                    if c != '.' {
                        symbols.insert(Point::new(x as i32, y as i32), c);
                    }
                }
            }
            if number_len > 0 {
                parts.insert(PartItem {
                    point: numberpoint,
                    length: number_len,
                    value: number,
                });
                number = 0;
                number_len = 0;
            }
        }
        let mut partmap = HashMap::new();
        for part in parts {
            let adj: HashSet<Point> = part
                .adjacent_points()
                .filter(|p| symbols.contains_key(p))
                .collect();
            partmap.insert(part, adj);
        }
        Ok(Self {
            parts: partmap,
            symbols,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.parts
            .iter()
            .filter(|(_, adjacent)| !adjacent.is_empty())
            .map(|(part, _)| part.value)
            .sum()
    }

    fn solve_part2(&self) -> u128 {
        self.symbols
            .iter()
            .filter(|(_, c)| **c == '*')
            .map(|(p, _)| {
                let adj_part_values: Vec<u64> = self
                    .parts
                    .iter()
                    .filter(|(_, value)| value.contains(p))
                    .map(|(part, _)| part.value)
                    .collect();
                if adj_part_values.len() == 2 {
                    u128::from(adj_part_values[0]) * u128::from(adj_part_values[1])
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    // Home made test to cover items in the rightmost column
    const TEST_CUSTOM_1: &str = ".23+..4.
.......*
11.....7
*.5..+..
3..2..*6";

    /// Leading zeros
    const TEST_CUSTOM_2: &str = "*012";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4361);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 54);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 467835);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 61);
    }
}
