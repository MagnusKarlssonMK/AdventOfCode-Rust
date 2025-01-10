//! 2024 day 25 - Code Chronicle
//!
//! Store the keys/locks as items represented by a number, made up of the depth of each column
//! bit-shifted into its location. The solution assumes width 5 and max depth 5 (top and bottom
//! rows are excluded), so each number is represented by a 4-bit number.
//!
//! Then when checking combinations of keys and locks, by adding the two numbers together, we can
//! check each column by making sure the bitmasked sum doesn't exceed 5. The easiest way to do this
//! is to add 2 and then see if it causes an overflow into the 4th bit (0x8). That way there is no
//! need to iterate over each individual digit.
use crate::aoc_util::grid::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((solution_data.solve_part1().to_string(), "-".to_string()))
}

enum Item {
    Lock(usize),
    Key(usize),
}

impl FromStr for Item {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = Grid::parse(s);
        // Naive approach - assume that all input items are correct, i.e. first line all have the same char
        let mut signature: Vec<usize> = vec![0; lines.x_max];
        let k = lines.elements[0];
        for (i, c) in lines.elements.iter().enumerate() {
            if *c == k {
                signature[i % lines.x_max] += 1;
            }
        }
        if k == '#' {
            Ok(Self::Lock(
                signature
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, n)| (*n - 1) << (4 * i))
                    .sum(),
            ))
        } else {
            Ok(Self::Key(
                signature
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, n)| (lines.y_max - *n - 1) << (4 * i))
                    .sum(),
            ))
        }
    }
}

struct InputData {
    locks: Vec<usize>,
    keys: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for item in s.split("\n\n").map(|i| Item::from_str(i).unwrap()) {
            match item {
                Item::Lock(s) => locks.push(s),
                Item::Key(s) => keys.push(s),
            }
        }
        Ok(Self { locks, keys })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut total = 0;
        for lock_sig in &self.locks {
            let padding = 0x22222;
            for key_sig in &self.keys {
                if (lock_sig + key_sig + padding) & 0x88888 == 0 {
                    total += 1;
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }
}
