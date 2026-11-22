//! # 2024 day 13 - Claw Contraption
//!
//! Calculates the answer basically by solving a matrix. The same function can be used for both
//! parts, with an argument to input the extra scaling value for part 2.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

/// Parses the last two whitespace-separated tokens of a line as a coordinate pair, taking the
/// number after `marker` in each token.
fn parse_coords(line: &str, marker: char) -> Result<(i64, i64), AocError> {
    let mut parts = line.split_whitespace().rev();
    let y = parts.next().ctx("missing y coordinate")?;
    let x = parts.next().ctx("missing x coordinate")?;

    let x = x
        .strip_suffix(',')
        .ctx("missing comma after x coordinate")?;
    let x = x.split_once(marker).ctx("missing x marker")?.1.parse()?;
    let y = y.split_once(marker).ctx("missing y marker")?.1.parse()?;
    Ok((x, y))
}

impl FromStr for ClawMachine {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Self {
            button_a: parse_coords(lines.next().ctx("missing button A line")?, '+')?,
            button_b: parse_coords(lines.next().ctx("missing button B line")?, '+')?,
            prize: parse_coords(lines.next().ctx("missing prize line")?, '=')?,
        })
    }
}

impl ClawMachine {
    fn get_win_tokens(&self, extra: i64) -> Option<u64> {
        let p_x = self.prize.0 + extra;
        let p_y = self.prize.1 + extra;
        let i = (self.button_b.0 * p_y - self.button_b.1 * p_x)
            / (self.button_a.1 * self.button_b.0 - self.button_a.0 * self.button_b.1);
        let j = (p_x - i * self.button_a.0) / self.button_b.0;
        if (p_x - i * self.button_a.0) % self.button_b.0 == 0
            && (self.button_b.0 * p_y - self.button_b.1 * p_x)
                % (self.button_a.1 * self.button_b.0 - self.button_a.0 * self.button_b.1)
                == 0
        {
            Some((3 * i + j) as u64)
        } else {
            None
        }
    }
}

struct InputData {
    machines: Vec<ClawMachine>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            machines: s
                .split("\n\n")
                .map(|m| m.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(|m| m.get_win_tokens(0))
            .sum()
    }

    fn solve_part2(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(|m| m.get_win_tokens(10_000_000_000_000))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 480);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 875318608908);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "Button A: X+1, Y+2",
            "Button A: X+1, Y+2\nButton B: X+3, Y+4",
            "Button A: X+1 Y+2\nButton B: X+3, Y+4\nPrize: X=5, Y=6",
            "Button A: X1, Y+2\nButton B: X+3, Y+4\nPrize: X=5, Y=6",
            "Button A: X+1, Y+2\nButton B: X+3, Y+4\nPrize: X=5, Y6",
        ] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }

        let input = "Button A: X+1, Y+2\nButton B: X+3, Y+4\nPrize: X=abc, Y=6";
        assert!(matches!(
            input.parse::<InputData>(),
            Err(AocError::ParseInt(_))
        ));
    }
}
