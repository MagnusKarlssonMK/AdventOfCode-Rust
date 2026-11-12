//! # 2021 day 4 - Giant Squid
//!
//! The entire draw is known ahead of time, so there is no need to simulate the bingo game
//! step-by-step; we can instead directly calculate which round a board will win and calculate its
//! score. The answer to part 1 is then simply the board that wins first, and for part 2 the board
//! that wins last.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve_parts1_2()?;
    Ok((p1.to_string(), p2.to_string()))
}

struct Board {
    rows: Vec<Vec<u32>>,
    cols: Vec<Vec<u32>>,
}

impl FromStr for Board {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<_>> = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        let width = rows.first().ctx_invalid("board row")?.len();
        if width == 0 {
            return Err(AocError::Invalid("empty board".to_string()));
        }
        if let Some(row) = rows.iter().find(|r| r.len() != width) {
            return Err(AocError::Invalid(format!(
                "board row has {} values, expected {width}",
                row.len(),
            )));
        }
        let cols = (0..width)
            .map(|col| rows.iter().map(|r| r[col]).collect())
            .collect();
        Ok(Self { rows, cols })
    }
}

impl Board {
    /// Returns (`round`, `score`) for the board, based on the input draw map. Returns None if it
    /// never wins.
    fn get_bingoround_and_score(&self, drawmap: &HashMap<u32, usize>) -> Option<(usize, u64)> {
        /// The round at which `line` is fully marked, or `None` if it contains a never-drawn number.
        fn line_win_round(line: &[u32], drawmap: &HashMap<u32, usize>) -> Option<usize> {
            line.iter().try_fold(0, |round, value| {
                let index = *drawmap.get(value)?;
                Some(round.max(index))
            })
        }
        let bingo_drawidx = self
            .rows
            .iter()
            .chain(self.cols.iter())
            .filter_map(|line| line_win_round(line, drawmap))
            .min()?;
        let score = self
            .rows
            .iter()
            .flatten()
            .filter(|&nbr| drawmap.get(nbr).is_none_or(|&i| i > bingo_drawidx))
            .map(|&nbr| u64::from(nbr))
            .sum();
        Some((bingo_drawidx, score))
    }
}

struct InputData {
    draw: Vec<u32>,
    boards: Vec<Board>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let draw: Vec<_> = blocks
            .next()
            .ctx("draw")?
            .split(',')
            .map(|c| c.parse())
            .collect::<Result<_, _>>()?;
        let boards: Vec<_> = blocks
            .filter(|b| !b.trim().is_empty())
            .map(|b| b.parse())
            .collect::<Result<_, _>>()?;
        if boards.is_empty() {
            Err(AocError::Invalid("no boards".to_string()))
        } else {
            Ok(Self { draw, boards })
        }
    }
}

impl InputData {
    fn solve_parts1_2(&self) -> Result<(u64, u64), AocError> {
        let drawmap: HashMap<_, _> = self.draw.iter().enumerate().map(|(i, d)| (*d, i)).collect();
        let results: Vec<(_, _)> = self
            .boards
            .iter()
            .filter_map(|board| board.get_bingoround_and_score(&drawmap))
            .collect();
        let first = results
            .iter()
            .min_by_key(|(round, _)| *round)
            .ok_or(AocError::NoSolution)?;
        let last = results
            .iter()
            .max_by_key(|(round, _)| *round)
            .ok_or(AocError::NoSolution)?;
        Ok((
            u64::from(self.draw[first.0]) * first.1,
            u64::from(self.draw[last.0]) * last.1,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    /// The first board wins via its second column; its first row contains the never-drawn 9, so that
    /// row cannot win and 9 must count as unmarked. The second board never wins and must be ignored.
    const TEST_CUSTOM_1: &str = "1,2,3

1 9
2 3

9 8
7 6";

    const TEST_CUSTOM_2: &str = "1,2,3

9 8
7 6";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve_parts1_2().unwrap();
        assert_eq!(p1, 4512);
        assert_eq!(p2, 1924);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        // Round 1 (draw 2): unmarked 9 and 3 -> score 12 -> 2 * 12 = 24.
        assert_eq!(solution_data.solve_parts1_2().unwrap(), (24, 24));
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_parts1_2(), Err(AocError::NoSolution));
    }

    #[test]
    fn parser_invalid_input() {
        assert!("1,2\n\n1 2\n3".parse::<InputData>().is_err()); // ragged rows
        assert!("1,2\n\n".parse::<InputData>().is_err()); // no boards
    }
}
