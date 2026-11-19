//! # 2023 day 13 - Point of Incidence
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

/// If `wildcard` == `true`, the wild card for part 2 has not yet been used
fn is_mirror(lines: &[u64], candidate_idx: isize, wildcard: bool) -> bool {
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
        // Only one difference (smudge) between this and next line. Consume the wildcard, create a
        // new list with these two removed and recurse.
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

fn get_mirror_score(lines: &[u64], wildcard: bool) -> usize {
    for idx in 0..lines.len() - 1 {
        if is_mirror(lines, idx as isize, wildcard) {
            return idx + 1;
        }
    }
    0
}

/// Each pattern is represented as a vector of numbers, where the binary 1 represents '#' and 0 '.'
struct Pattern {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl FromStr for Pattern {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let nbr_rows = lines.len();
        let nbr_cols = lines.first().ctx("empty pattern")?.len();
        if lines.iter().any(|line| line.len() != nbr_cols) {
            return Err(AocError::Invalid("jagged lines in pattern".to_string()));
        }
        if nbr_cols > u64::BITS as usize || nbr_rows > u64::BITS as usize {
            return Err(AocError::Invalid("pattern too big".to_string()));
        }
        let mut rows = vec![0; nbr_rows];
        let mut cols = vec![0; nbr_cols];
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    rows[row] += 1 << (nbr_cols - col - 1);
                    cols[col] += 1 << (nbr_rows - row - 1);
                } else if c != '.' {
                    return Err(AocError::Invalid(format!(
                        "invalid character {c:?} in {line:?}"
                    )));
                }
            }
        }
        Ok(Self { rows, cols })
    }
}

impl Pattern {
    /// Rows and columns are scored independently, each with its own single allowed smudge. For
    /// part 2 this relies on the puzzle's guarantee that each pattern has exactly one smudge; a
    /// row mirror and a column mirror could otherwise require two different smudges.
    fn get_score(&self, wildcard: bool) -> usize {
        100 * get_mirror_score(&self.rows, wildcard) + get_mirror_score(&self.cols, wildcard)
    }
}

struct InputData {
    patterns: Vec<Pattern>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            patterns: s
                .split("\n\n")
                .map(|block| block.parse())
                .collect::<Result<_, _>>()?,
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

    const TEST_OFFICIAL_1: &str = "#.##..##.
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
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 405);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 400);
    }

    #[test]
    fn parser_rejects_empty_pattern() {
        assert!(matches!(
            "".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn parser_rejects_jagged_lines() {
        assert!(matches!(
            "#.#\n.##\n#.".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn parser_rejects_invalid_character() {
        assert!(matches!(
            "#.#\n.x.\n...".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn parser_rejects_too_many_rows() {
        let input = vec![".#"; 65].join("\n");
        assert!(matches!(
            input.parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn parser_rejects_too_many_cols() {
        let input = ".".repeat(65);
        assert!(matches!(
            input.parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn parser_accepts_max_dimensions() {
        let input = vec![".".repeat(64); 64].join("\n");
        assert!(input.parse::<InputData>().is_ok());
    }
}
