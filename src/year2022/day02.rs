//! # 2022 day 2 - Rock Paper Scissors
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Hand {
    fn get_val(&self) -> usize {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissors => 2,
        }
    }

    fn from_val(v: usize) -> Self {
        match v {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn get_score(&self, other: &Self) -> usize {
        if self == other {
            3 + other.get_val() + 1
        } else if other.get_val() == (self.get_val() + 1) % 3 {
            6 + other.get_val() + 1
        } else {
            other.get_val() + 1
        }
    }

    fn determine_response(&self, other: &Guide) -> Self {
        match other {
            Guide::Lose => Hand::from_val((self.get_val() + 2) % 3),
            Guide::Win => Hand::from_val((self.get_val() + 1) % 3),
            Guide::Draw => Hand::from_val(self.get_val()),
        }
    }
}

enum Guide {
    Lose,
    Win,
    Draw,
}

impl FromStr for Guide {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Guide {
    fn as_hand(&self) -> Hand {
        match self {
            Self::Lose => Hand::Rock,
            Self::Draw => Hand::Paper,
            Self::Win => Hand::Scissors,
        }
    }
}

struct InputData {
    rounds: Vec<(Hand, Guide)>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rounds: s
                .lines()
                .map(|line| {
                    let hands: Vec<&str> = line.split_whitespace().collect();
                    (
                        Hand::from_str(hands[0]).unwrap(),
                        Guide::from_str(hands[1]).unwrap(),
                    )
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.rounds
            .iter()
            .map(|(h, g)| h.get_score(&g.as_hand()))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.rounds
            .iter()
            .map(|(h, g)| h.get_score(&h.determine_response(g)))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 15);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 12);
    }
}
