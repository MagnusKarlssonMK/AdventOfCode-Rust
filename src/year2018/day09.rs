//! # 2018 day 9 - Marble Mania
//!
//! Simply runs the simulation according to the problem description, but rather than keeping
//! track of current position, the circle (represented by a VecDeque) is rotated so that the
//! current position is always the first element. Not the most efficient way to do it, but
//! gets the job done in a decent time.
use crate::aoc_util::error::*;
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const MULTIPLE: usize = 23;
const REMOVE: usize = 7;

struct InputData {
    players: usize,
    points: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            [
                players,
                "players;",
                "last",
                "marble",
                "is",
                "worth",
                points,
                "points",
            ] => {
                let players = players.parse()?;
                if players > 0 {
                    Ok(Self {
                        players,
                        points: points.parse()?,
                    })
                } else {
                    Err(AocError::Invalid("players = 0".to_string()))
                }
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl InputData {
    fn get_winning_score(&self, marble_multiplier: usize) -> usize {
        let mut scores = vec![0usize; self.players];
        let mut marble_circle = VecDeque::from([0usize]);
        for marble in 1..=(self.points * marble_multiplier) {
            if !marble.is_multiple_of(MULTIPLE) {
                marble_circle.rotate_left(1);
                marble_circle.push_back(marble);
            } else {
                marble_circle.rotate_right(REMOVE);
                let player = marble % self.players;
                scores[player] += marble
                    + marble_circle
                        .pop_back()
                        .expect("marble circle should never be empty");
                marble_circle.rotate_left(1);
            }
        }
        *scores
            .iter()
            .max()
            .expect("player scores should never be empty")
    }

    fn solve_part1(&self) -> usize {
        self.get_winning_score(1)
    }

    fn solve_part2(&self) -> usize {
        self.get_winning_score(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "10 players; last marble is worth 1618 points";
    const TEST_OFFICIAL_2: &str = "13 players; last marble is worth 7999 points";
    const TEST_OFFICIAL_3: &str = "17 players; last marble is worth 1104 points";
    const TEST_OFFICIAL_4: &str = "21 players; last marble is worth 6111 points";
    const TEST_OFFICIAL_5: &str = "30 players; last marble is worth 5807 points";

    const TEST_CUSTOM_1: &str = "9 players; last marble is worth 25 points";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 8317);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 146373);
    }

    #[test]
    fn part1_official_3() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 2764);
    }

    #[test]
    fn part1_official_4() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_4).unwrap();
        assert_eq!(solution_data.solve_part1(), 54718);
    }

    #[test]
    fn part1_official_5() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_5).unwrap();
        assert_eq!(solution_data.solve_part1(), 37305);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 74765078);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 32);
        assert_eq!(solution_data.solve_part2(), 22563);
    }
}
