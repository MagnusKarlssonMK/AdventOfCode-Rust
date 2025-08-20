//! # 2023 day 7 - Camel Cards
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

enum HandResult {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandResult {
    /// Takes an array of sorted card counts (highest counts first) and calculates the hand result
    fn get_hand_result(counts: &[u8]) -> Self {
        match counts[0] {
            5 => HandResult::FiveOfAKind,
            4 => HandResult::FourOfAKind,
            3 => {
                if counts[1] == 2 {
                    HandResult::FullHouse
                } else {
                    HandResult::ThreeOfAKind
                }
            }
            2 => {
                if counts[1] == 2 {
                    HandResult::TwoPairs
                } else {
                    HandResult::OnePair
                }
            }
            1 => HandResult::HighCard,
            _ => unreachable!(),
        }
    }
}

/// Stores the hand in a vec of cards, its bid, and an array of the number of cards
/// for each card type. The cards T, J, Q, K, A are stored as hex numbers in that order.
struct Hand {
    cards: Vec<u8>,
    count: [u8; 15],
    bid: u32,
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').unwrap();
        let bid = right.parse().unwrap();
        let cards: Vec<u8> = left
            .chars()
            .map(|c| {
                if c.is_numeric() {
                    c.to_digit(10).unwrap() as u8
                } else {
                    match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => unreachable!(),
                    }
                }
            })
            .collect();
        let mut count = [0; 15];
        for c in &cards {
            count[*c as usize] += 1;
        }
        Ok(Self { cards, count, bid })
    }
}

impl Hand {
    /// Calculates a comparabe power value for the hand by putting the card values as hex numbers
    /// in the 5 first nibbles, and the hand power value in the 6th nibble.
    fn get_power(&self) -> usize {
        let mut sorted_counts = self.count;
        sorted_counts.sort_unstable_by(|a, b| b.cmp(a));
        let handtype = HandResult::get_hand_result(&sorted_counts) as usize;
        let mut power = handtype << (5 * 4);
        for (i, c) in self.cards.iter().rev().enumerate() {
            power += (*c as usize) << (i * 4);
        }
        power
    }

    /// Same as get_power(), but uses J as jokers. Jokers gets the card value of 1 instead of 0xb
    fn get_joker_power(&self) -> usize {
        let mut sorted_counts = self.count;
        let joker_count = sorted_counts[0xb];
        sorted_counts[0xb] = 0;
        let handtype = if joker_count >= 4 {
            HandResult::FiveOfAKind as usize
        } else {
            sorted_counts.sort_unstable_by(|a, b| b.cmp(a));
            sorted_counts[0] += joker_count;
            HandResult::get_hand_result(&sorted_counts) as usize
        };
        let mut power = handtype << (5 * 4);
        for (i, c) in self.cards.iter().rev().enumerate() {
            if *c == 0xb {
                power += 1 << (i * 4);
            } else {
                power += (*c as usize) << (i * 4);
            }
        }
        power
    }
}

struct InputData {
    hands: Vec<Hand>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            hands: s
                .lines()
                .map(|line| Hand::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut powers: Vec<(usize, u32)> =
            self.hands.iter().map(|h| (h.get_power(), h.bid)).collect();
        powers.sort_unstable();
        powers
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| *bid as usize * (rank + 1))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut powers: Vec<(usize, u32)> = self
            .hands
            .iter()
            .map(|h| (h.get_joker_power(), h.bid))
            .collect();
        powers.sort_unstable();
        powers
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| *bid as usize * (rank + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.hands[0].get_power(), 0x232a3d);
        assert_eq!(solution_data.hands[1].get_power(), 0x4a55b5);
        assert_eq!(solution_data.hands[2].get_power(), 0x3dd677);
        assert_eq!(solution_data.hands[3].get_power(), 0x3dabba);
        assert_eq!(solution_data.hands[4].get_power(), 0x4cccbe);
        assert_eq!(solution_data.solve_part1(), 6440);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.hands[0].get_joker_power(), 0x232a3d);
        assert_eq!(solution_data.hands[1].get_joker_power(), 0x6a55b5);
        assert_eq!(solution_data.hands[2].get_joker_power(), 0x3dd677);
        assert_eq!(solution_data.hands[3].get_joker_power(), 0x6dabba);
        assert_eq!(solution_data.hands[4].get_joker_power(), 0x6cccbe);
        assert_eq!(solution_data.solve_part2(), 5905);
    }
}
