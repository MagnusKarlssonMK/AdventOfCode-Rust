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
use crate::aoc_util::error::{AocError, OptionExt};
use std::str::FromStr;

const WIDTH: usize = 5;
const MAX_DEPTH: usize = 5;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((solution_data.solve_part1().to_string(), "-".to_string()))
}

enum Item {
    Lock(usize),
    Key(usize),
}

impl FromStr for Item {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let first = lines.first().ctx("empty item")?;
        if lines.iter().any(|line| line.chars().count() != WIDTH) {
            return Err(AocError::Invalid(format!(
                "item rows must be {WIDTH} characters wide"
            )));
        }
        let k = first.chars().next().ctx("empty item row")?;
        if !matches!(k, '#' | '.') {
            return Err(AocError::Invalid(format!("invalid item character: {k}")));
        }

        let mut counts = [0usize; WIDTH];
        for line in &lines {
            for (x, c) in line.chars().enumerate() {
                if c == k {
                    counts[x] += 1;
                }
            }
        }

        let depths: Vec<usize> = if k == '#' {
            counts
                .iter()
                .map(|n| n.checked_sub(1).ctx("lock column without '#'"))
                .collect::<Result<_, _>>()?
        } else {
            counts
                .iter()
                .map(|n| {
                    lines
                        .len()
                        .checked_sub(*n + 1)
                        .ctx("key column without '#'")
                })
                .collect::<Result<_, _>>()?
        };
        if depths.iter().any(|&d| d > MAX_DEPTH) {
            return Err(AocError::Invalid(format!("item depth exceeds {MAX_DEPTH}")));
        }
        let signature: usize = depths
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, n)| n << (4 * i))
            .sum();
        Ok(if k == '#' {
            Self::Lock(signature)
        } else {
            Self::Key(signature)
        })
    }
}

struct InputData {
    locks: Vec<usize>,
    keys: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for item in s.split("\n\n").map(Item::from_str) {
            match item? {
                Item::Lock(s) => locks.push(s),
                Item::Key(s) => keys.push(s),
            }
        }
        Ok(Self { locks, keys })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        const PADDING: usize = 0x22222;
        const MASK: usize = 0x88888;
        self.locks
            .iter()
            .flat_map(|lock| {
                self.keys
                    .iter()
                    .filter(|key| (*lock + **key + PADDING) & MASK == 0)
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "#####
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

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "####\n....",                                      // wrong row width
            "X....\n.....",                                    // invalid first character
            "##.##\n##.##\n##.##\n##.##\n##.##\n##.##\n##.##", // lock column without '#'
            ".....\n.....\n.....\n.....\n.....\n.....\n.....", // key column without '#'
            ".#...\n.#...\n.#...\n.#...\n.#...\n.#...\n.#...", // key column without '.'
            "#####\n#####\n#####\n#####\n#####\n#####\n#####", // depth exceeds MAX_DEPTH
        ] {
            assert!(matches!(input.parse::<Item>(), Err(AocError::Invalid(_))));
        }
    }
}
