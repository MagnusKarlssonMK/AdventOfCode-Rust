//! # 2021 day 10 - Syntax Scoring
//!
//! Parses each navigation line with a stack of opening-bracket indices. A mismatched closer gives
//! the syntax-error score for part 1; leftover openings on an otherwise valid line give the
//! completion score for part 2, whose median is the answer.
use crate::aoc_util::error::AocError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let p1 = solution_data.solve_part1();
    let p2 = solution_data.solve_part2()?;
    Ok((p1.to_string(), p2.to_string()))
}

const OPENING_BRACKETS: [char; 4] = ['(', '[', '{', '<'];
const CLOSING_BRACKETS: [char; 4] = [')', ']', '}', '>'];
const SYNTAX_SCORES: [u64; 4] = [3, 57, 1197, 25137];
const COMPLETION_SCORES: [u64; 4] = [1, 2, 3, 4];

fn opening_index(c: char) -> Option<usize> {
    OPENING_BRACKETS.iter().position(|&x| x == c)
}

fn closing_index(c: char) -> Option<usize> {
    CLOSING_BRACKETS.iter().position(|&x| x == c)
}

#[derive(Debug)]
enum NavigationLine {
    Valid,
    Incomplete(u64),
    Corrupt(u64),
}

impl FromStr for NavigationLine {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some(i) = opening_index(c) {
                stack.push(i);
            } else if let Some(i) = closing_index(c) {
                if stack.last() == Some(&i) {
                    stack.pop();
                } else {
                    return Ok(NavigationLine::Corrupt(SYNTAX_SCORES[i]));
                }
            } else {
                return Err(AocError::Invalid(format!("invalid character: {c}")));
            }
        }
        if stack.is_empty() {
            Ok(NavigationLine::Valid)
        } else {
            let completion_score = stack.iter().rev().fold(0, |score, &i| {
                score * 5 + COMPLETION_SCORES[i]
            });
            Ok(NavigationLine::Incomplete(completion_score))
        }
    }
}

struct InputData {
    lines: Vec<NavigationLine>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.lines
            .iter()
            .filter_map(|line| match line {
                NavigationLine::Corrupt(v) => Some(*v),
                _ => None,
            })
            .sum()
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let mut scores: Vec<_> = self
            .lines
            .iter()
            .filter_map(|line| match line {
                NavigationLine::Incomplete(v) => Some(*v),
                _ => None,
            })
            .collect();
        if scores.is_empty() {
            return Err(AocError::NoSolution);
        }
        scores.sort_unstable();
        Ok(scores[scores.len() / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    const TEST_CUSTOM_1: &str = "()
)
[(
<(
{[
[<
<]";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 26397);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 60);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 288957);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 13);
    }

    #[test]
    fn parser_invalid_input() {
        let err = "([x])".parse::<InputData>().err().unwrap();
        assert_eq!(err, AocError::Invalid("invalid character: x".to_string()));
    }

    #[test]
    fn part2_no_solution() {
        let solution_data: InputData = "()\n)".parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap_err(), AocError::NoSolution);
    }
}
