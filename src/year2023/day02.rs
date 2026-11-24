//! # 2023 day 2 - Cube Conundrum
//!
//! The bag holds 12 red, 14 blue and 13 green cubes in total, and every game is split into sets
//! of cubes. For part 1, sum the ids of the games where each single set of the game fits inside
//! the bag. For part 2, ignore the bag: require only the largest number of cubes of each color
//! seen in any set of the game, and the power of the game is the product of those three values.
use crate::aoc_util::error::{AocError, OptionExt};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const VALID_BAG: CubeSet = CubeSet {
    red: 12,
    blue: 14,
    green: 13,
};

/// The number of cubes of each color in one set.
#[derive(Default)]
struct CubeSet {
    red: u16,
    blue: u16,
    green: u16,
}

impl FromStr for CubeSet {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0u16;
        let mut blue = 0u16;
        let mut green = 0u16;
        for cube in s.split(", ") {
            let (nbr, color) = cube.split_once(' ').ok_or_else(|| {
                AocError::Invalid(format!("expected '<count> <color>': {cube:?}"))
            })?;
            let count = match color {
                "red" => &mut red,
                "blue" => &mut blue,
                "green" => &mut green,
                _ => return Err(AocError::Invalid(format!("unknown color: {color:?}"))),
            };
            *count = count
                .checked_add(nbr.parse()?)
                .ok_or_else(|| AocError::Overflow("cube count overflow".to_string()))?;
        }
        Ok(Self { red, green, blue })
    }
}

impl CubeSet {
    #[inline]
    fn is_valid(&self) -> bool {
        self.red <= VALID_BAG.red && self.blue <= VALID_BAG.blue && self.green <= VALID_BAG.green
    }

    #[inline]
    fn get_power(&self) -> u64 {
        u64::from(self.red) * u64::from(self.blue) * u64::from(self.green)
    }

    #[inline]
    fn get_max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
}

struct Game {
    game_id: u16,
    cube_sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").ctx("expected 'Game <id>: <sets>'")?;
        Ok(Self {
            game_id: left
                .strip_prefix("Game ")
                .ctx("expected 'Game <id>'")?
                .parse()?,
            cube_sets: right
                .split("; ")
                .map(|c| c.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.cube_sets.iter().all(|set| set.is_valid())
    }

    fn get_power(&self) -> u64 {
        self.cube_sets
            .iter()
            .fold(CubeSet::default(), |acc, c| acc.get_max(c))
            .get_power()
    }
}

struct InputData {
    games: Vec<Game>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            games: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u32 {
        self.games
            .iter()
            .filter(|g| g.is_valid())
            .map(|g| u32::from(g.game_id))
            .sum()
    }

    fn solve_part2(&self) -> u64 {
        self.games.iter().map(|g| g.get_power()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    /// Bag boundaries: 13 red does not fit, while 12 red, 14 blue and 13 green are all exactly
    /// within the bag. A game that never mentions a color holds none of it, so its part 2 power
    /// is zero.
    const TEST_CUSTOM_1: &str = "Game 7: 13 red
Game 8: 12 red, 14 blue, 13 green";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 8);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 2286);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 8);
        assert_eq!(solution_data.solve_part2(), 2184);
    }

    #[test]
    fn parser_invalid_input() {
        for line in [
            "Game 1 3 blue",
            "Foo 1: 3 blue",
            "Game 1: 3 purple",
            "Game 1: 3 blue; 2",
        ] {
            assert!(matches!(line.parse::<Game>(), Err(AocError::Invalid(_))));
        }
    }

    #[test]
    fn parser_parse_int() {
        for line in ["Game x: 3 blue", "Game 1: 65537 red, 1 red"] {
            assert!(matches!(line.parse::<Game>(), Err(AocError::ParseInt(_))));
        }
    }

    #[test]
    fn parser_overflow() {
        assert!(matches!(
            "Game 1: 65535 red, 1 red".parse::<Game>(),
            Err(AocError::Overflow(_))
        ));
    }
}
