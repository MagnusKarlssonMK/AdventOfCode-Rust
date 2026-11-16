//! # 2023 day 4 - Scratchcards
//!
//! Each card holds a set of winning numbers and the numbers that were drawn for it, and the
//! matches of the card are the numbers that appear in both sets. For part 1, a card with matches
//! is worth 2^(matches - 1) points, while a card without a single match is worth nothing. For
//! part 2, a card hands out one extra copy of each of the next matches cards, and the answer is
//! the total number of copies once every card has handed out its copies in card order.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashSet, error::Error, str::FromStr, vec};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Card {
    wincount: usize,
    score: u64,
}

impl FromStr for Card {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Note: solution identifies card by index, so Card ID is irrelevant
        let (_, nbrs) = s
            .strip_prefix("Card ")
            .and_then(|rest| rest.split_once(": "))
            .ctx("expected 'Card <id>: <numbers>'")?;
        let (left, right) = nbrs.split_once(" | ").ctx("card | delimiter")?;
        let winning_nbrs: HashSet<u8> = left
            .split_whitespace()
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;
        let draw_nbrs: HashSet<u8> = right
            .split_whitespace()
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;
        let wincount = winning_nbrs.intersection(&draw_nbrs).count();
        let score = if wincount > 0 {
            1u64.checked_shl(wincount as u32 - 1)
                .ok_or_else(|| AocError::Overflow("card score overflow".to_string()))?
        } else {
            0
        };
        Ok(Self { wincount, score })
    }
}

struct InputData {
    scratchcards: Vec<Card>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            scratchcards: s
                .lines()
                .map(|card| card.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.scratchcards.iter().map(|c| c.score).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut copylist = vec![1; self.scratchcards.len()];
        for (i, card) in self.scratchcards.iter().enumerate() {
            let copies = copylist[i];
            for slot in copylist.iter_mut().skip(i + 1).take(card.wincount) {
                *slot += copies;
            }
        }
        copylist.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    /// A single match scores 1 and no match scores 0, and every card counts itself in part 2:
    /// card 1 hands its one copy to card 2, so the two cards add up to three copies.
    const TEST_CUSTOM_1: &str = "Card 1: 1 2 3 | 1 5 9
Card 2: 4 5 6 | 1 2 3";

    /// Card 1 wins a copy of card 2, and the would-be card 3 does not exist.
    const TEST_CUSTOM_2: &str = "Card 1: 1 2 | 1 2
Card 2: 3 | 4";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 30);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn parser_invalid_input() {
        for line in ["Card 1 41 48", "Card 1: 41 48", "Foo 1: 1 | 1"] {
            assert!(matches!(line.parse::<Card>(), Err(AocError::Invalid(_))));
        }
    }
}
