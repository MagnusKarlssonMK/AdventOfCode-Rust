//! # 2023 day 13 - Point of Incidence
use std::{error::Error, str::FromStr, vec};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

/// If [wildcard] == [true], the wild card for part 2 is still not used
fn is_mirror(lines: &[usize], candidate_idx: isize, wildcard: bool) -> bool {
    if !(0..(lines.len() as isize) - 1).contains(&candidate_idx) {
        !wildcard
    } else if lines[candidate_idx as usize] == lines[candidate_idx as usize + 1] {
        // Identical to next line - create a new list with these two removed and recurse.
        let new_lines: Vec<_> = lines
            .iter()
            .enumerate()
            .filter(|(i, _)| !(candidate_idx..candidate_idx + 2).contains(&(*i as isize)))
            .map(|(_, v)| *v)
            .collect();
        is_mirror(&new_lines, candidate_idx - 1, wildcard)
    } else if wildcard
        && (lines[candidate_idx as usize] ^ lines[candidate_idx as usize + 1]).count_ones() == 1
    {
        // Only one difference (smudge) between this and next line. Consume the wildcard, create a new
        // list with these two removed and recurse.
        let new_lines: Vec<_> = lines
            .iter()
            .enumerate()
            .filter(|(i, _)| !(candidate_idx..candidate_idx + 2).contains(&(*i as isize)))
            .map(|(_, v)| *v)
            .collect();
        is_mirror(&new_lines, candidate_idx - 1, false)
    } else {
        false
    }
}

fn get_mirror_score(lines: &[usize], wildcard: bool) -> usize {
    for idx in 0..lines.len() - 1 {
        if is_mirror(lines, idx as isize, wildcard) {
            return idx + 1;
        }
    }
    0
}

/// Each pattern is represented as a vector of numbers, where the binary 1 represents '#' and 0 '.'
struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl FromStr for Pattern {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nbr_rows = s.lines().count();
        let nbr_cols = s.lines().last().unwrap().len();
        let mut rows = vec![0; nbr_rows];
        let mut cols = vec![0; nbr_cols];
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    rows[row] += 1 << (nbr_cols - col - 1);
                    cols[col] += 1 << (nbr_rows - row - 1);
                }
            }
        }
        Ok(Self { rows, cols })
    }
}

impl Pattern {
    fn get_score(&self, wildcard: bool) -> usize {
        100 * get_mirror_score(&self.rows, wildcard) + get_mirror_score(&self.cols, wildcard)
    }
}

struct InputData {
    patterns: Vec<Pattern>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            patterns: s
                .split("\n\n")
                .map(|block| Pattern::from_str(block).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.patterns.iter().map(|p| p.get_score(false)).sum()
    }

    fn solve_part2(&self) -> usize {
        self.patterns.iter().map(|p| p.get_score(true)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 405);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 400);
    }
}
