//! # 2016 day 17 - Two Steps Forward
//!
//! A brute force BFS solution exploring every possible path, using a state consisting
//! of the current point and the path taken to get there.
use crate::aoc_util::{error::*, point::*};
use md5::{Digest, Md5};
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1, p2.to_string()))
}

const GRID_SIZE: i32 = 4;
const TARGET: Point = Point::new(GRID_SIZE - 1, GRID_SIZE - 1);

/// Character to direction mapping.
/// The order must match the puzzle's Up/Down/Left/Right, since each hex digit of the hash maps to the direction in that order.
const DIRECTIONS: [(char, Point); 4] = [('U', UP), ('D', DOWN), ('L', LEFT), ('R', RIGHT)];

/// The `index`-th hex digit of `digest`, as a nibble value (0..=15).
fn hex_nibble(digest: &[u8], index: usize) -> u8 {
    let byte = digest[index / 2];
    if index.is_multiple_of(2) {
        byte >> 4
    } else {
        byte & 0x0f
    }
}

struct State {
    point: Point,
    path: String,
}

impl State {
    fn get_open_neighbors(&self, passcode: &str) -> Vec<State> {
        let mut result = Vec::new();
        let mut hasher = Md5::new();
        hasher.update(passcode);
        hasher.update(self.path.as_bytes());
        let digest = hasher.finalize();
        for (i, (c, d)) in DIRECTIONS.iter().enumerate() {
            let np = self.point + *d;
            if (0..GRID_SIZE).contains(&np.x)
                && (0..GRID_SIZE).contains(&np.y)
                && hex_nibble(&digest, i) >= 0xb
            {
                result.push(State {
                    point: np,
                    path: format!("{}{}", self.path, c),
                });
            }
        }
        result
    }
}

struct InputData {
    passcode: String,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            passcode: s.to_string(),
        })
    }
}

impl InputData {
    /// BFS every path to the vault; returns the shortest path and the longest path length.
    fn solve(&self) -> Result<(String, usize), AocError> {
        let mut queue = VecDeque::from([State {
            point: ORIGIN,
            path: "".to_string(),
        }]);
        let mut shortest = None;
        let mut longest = 0;
        while let Some(state) = queue.pop_front() {
            if state.point == TARGET {
                if shortest.is_none() {
                    shortest = Some(state.path.clone());
                }
                // BFS pops in non-decreasing order, so the longest path is always the last one found
                longest = state.path.len();
                continue;
            }
            queue.extend(state.get_open_neighbors(&self.passcode));
        }
        let p1 = shortest.ok_or(AocError::NoSolution)?;
        Ok((p1, longest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str("ihgpwlah").unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, "DDRRRD");
        assert_eq!(p2, 370);
    }

    #[test]
    fn part1_2_official_2() {
        let solution_data = InputData::from_str("kglvqrro").unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, "DDUDRLRRUDRD");
        assert_eq!(p2, 492);
    }

    #[test]
    fn part1_2_official_3() {
        let solution_data = InputData::from_str("ulqzkmiv").unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
        assert_eq!(p2, 830);
    }
}
