//! # 2023 day 3 - Gear Ratios
use crate::aoc_util::point::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct PartItem {
    point: Point,
    length: usize,
    value: usize,
}

impl PartItem {
    fn get_adjacent_points(&self) -> Vec<Point> {
        let mut n = Vec::new();
        for y in self.point.y - 1..=self.point.y + 1 {
            for x in self.point.x - 1..=self.point.x + self.length as i32 {
                n.push(Point::new(x, y));
            }
        }
        n
    }
}

struct InputData {
    parts: HashMap<PartItem, HashSet<Point>>,
    symbols: HashMap<Point, char>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number: usize = 0;
        let mut numberpoint = Point::new(0, 0);
        let mut parts = HashSet::new();
        let mut symbols = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if number == 0 {
                        numberpoint.x = x as i32;
                        numberpoint.y = y as i32;
                    }
                    number = 10 * number + c.to_digit(10).unwrap() as usize;
                } else {
                    if number > 0 {
                        parts.insert(PartItem {
                            point: numberpoint,
                            length: x - numberpoint.x as usize,
                            value: number,
                        });
                        number = 0;
                    }
                    if c != '.' {
                        symbols.insert(Point::new(x as i32, y as i32), c);
                    }
                }
            }
            if number > 0 {
                parts.insert(PartItem {
                    point: numberpoint,
                    length: line.len() - numberpoint.x as usize,
                    value: number,
                });
                number = 0;
            }
        }
        let mut partmap = HashMap::new();
        for part in parts {
            let mut adj = HashSet::new();
            for p in part.get_adjacent_points().iter() {
                if symbols.contains_key(p) {
                    adj.insert(*p);
                }
            }
            partmap.insert(part, adj);
        }
        Ok(Self {
            parts: partmap,
            symbols,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.parts
            .iter()
            .map(|(p, v)| if !v.is_empty() { p.value } else { 0 })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.symbols
            .iter()
            .filter(|(_, c)| **c == '*')
            .map(|(p, _)| {
                let adj_part_values: Vec<usize> = self
                    .parts
                    .iter()
                    .filter(|(_, value)| value.contains(p))
                    .map(|(part, _)| part.value)
                    .collect();
                if adj_part_values.len() == 2 {
                    adj_part_values[0] * adj_part_values[1]
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

    const TEST_DATA: &str = "467..114..
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
    const TEST_DATA_CUSTOM: &str = ".23+..4.
.......*
11.....7
*.5..+..
3..2..*6";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 4361);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_CUSTOM).unwrap();
        assert_eq!(solution_data.solve_part1(), 54);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 467835);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_CUSTOM).unwrap();
        assert_eq!(solution_data.solve_part2(), 61);
    }
}
