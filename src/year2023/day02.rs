//! # 2023 day 2 - Cube Conundrum
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Hand {
    red: usize,
    blue: usize,
    green: usize,
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for n in s.split(", ") {
            let (nbr, color) = n.split_once(' ').unwrap();
            match color.chars().nth(0) {
                Some('r') => red += nbr.parse::<usize>().unwrap(),
                Some('b') => blue += nbr.parse::<usize>().unwrap(),
                Some('g') => green += nbr.parse::<usize>().unwrap(),
                _ => return Err(()),
            }
        }
        Ok(Self { red, green, blue })
    }
}

impl Hand {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.blue <= 14 && self.green <= 13
    }

    fn get_power(&self) -> usize {
        self.red * self.blue * self.green
    }

    fn get_max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
}

struct Game {
    game_id: usize,
    hands: Vec<Hand>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let (_, gid) = left.split_once(' ').unwrap();
        Ok(Self {
            game_id: gid.parse().unwrap(),
            hands: right
                .split("; ")
                .map(|h| Hand::from_str(h).unwrap())
                .collect(),
        })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.hands.iter().all(|h| h.is_valid())
    }

    fn get_power(&self) -> usize {
        let mut minimum_required = Hand {
            red: 0,
            blue: 0,
            green: 0,
        };
        for h in self.hands.iter() {
            minimum_required = h.get_max(&minimum_required);
        }
        minimum_required.get_power()
    }
}

struct InputData {
    games: Vec<Game>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            games: s
                .lines()
                .map(|line| Game::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.games
            .iter()
            .filter(|g| g.is_valid())
            .map(|g| g.game_id)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.games.iter().map(|g| g.get_power()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 8);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 2286);
    }
}
