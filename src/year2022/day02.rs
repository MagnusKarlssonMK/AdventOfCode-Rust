//! # 2022 day 2 - Rock Paper Scissors
//!
//! Hands are encoded as positions in the three-step rock-paper-scissors cycle: Rock = 0,
//! Paper = 1, Scissors = 2. A player wins when their hand is one step ahead of the opponent's.
//! The X/Y/Z column is interpreted either as a hand for part 1, or as the desired outcome
//! (lose/draw/win) for part 2.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl FromStr for Hand {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Hand {
    const ALL: [Self; 3] = [Self::Rock, Self::Paper, Self::Scissors];

    /// Numeric position in the rock-paper-scissors cycle.
    fn value(self) -> u8 {
        self as u8
    }

    /// Converts a numeric position back to a hand, wrapping modulo 3.
    fn from_value(v: u8) -> Self {
        Self::ALL[(v % 3) as usize]
    }

    /// Score for playing `self` against `opponent`.
    fn score(self, opponent: Self) -> u64 {
        let shape_score = u64::from(self.value()) + 1;
        match (self.value() + 3 - opponent.value()) % 3 {
            0 => shape_score + 3, // draw
            1 => shape_score + 6, // win
            _ => shape_score,     // loss
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Guide {
    X = 0,
    Y = 1,
    Z = 2,
}

impl FromStr for Guide {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Guide {
    /// Part 1: the second column is our hand (X/Y/Z = Rock/Paper/Scissors).
    fn as_hand(self) -> Hand {
        Hand::from_value(self as u8)
    }

    /// Part 2: the second column is the desired outcome (X/Y/Z = lose/draw/win),
    /// which is an offset of 2/0/1 from the opponent's hand.
    fn as_response(self, opponent: Hand) -> Hand {
        Hand::from_value(opponent.value() + self as u8 + 2)
    }
}

struct InputData {
    rounds: Vec<(Hand, Guide)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds: Vec<(Hand, Guide)> = s
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let opponent = parts.next().ctx(format!("missing opponent in {line:?}"))?;
                let guide = parts.next().ctx(format!("missing guide in {line:?}"))?;
                Ok((opponent.parse()?, guide.parse()?))
            })
            .collect::<Result<_, AocError>>()?;
        Ok(Self { rounds })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.rounds
            .iter()
            .map(|(opponent, guide)| guide.as_hand().score(*opponent))
            .sum()
    }

    fn solve_part2(&self) -> u64 {
        self.rounds
            .iter()
            .map(|(opponent, guide)| guide.as_response(*opponent).score(*opponent))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "A Y\nB X\nC Z";

    const TEST_CUSTOM_1: &str = "A X\nA X\nA Y\nA Z";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 15);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 12);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 19);
        assert_eq!(solution_data.solve_part2(), 18);
    }

    #[test]
    fn parser_invalid_input() {
        assert!(matches!(
            "A".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
        assert!(matches!(
            "D X".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
        assert!(matches!(
            "A X\nB".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }
}
