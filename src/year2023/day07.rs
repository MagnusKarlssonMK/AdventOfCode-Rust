//! # 2023 day 7 - Camel Cards
use crate::aoc_util::error::AocError;
use std::str::FromStr;

/// The part 1 value of the `J` card; in part 2 it is a wild card instead.
const JOKER: u8 = 11;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
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

/// Stores the five cards (2..=14, with T=10, J=11, Q=12, K=13, A=14) and the bid.
struct Hand {
    cards: [u8; 5],
    bid: u32,
}

/// Maps a card character to its part 1 value, or `None` if it is not a valid card.
fn card_value(c: char) -> Option<u8> {
    match c {
        '2'..='9' => Some(c as u8 - b'0'),
        'T' => Some(10),
        'J' => Some(JOKER),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

impl FromStr for Hand {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or_else(|| AocError::Invalid(format!("card: {s}")))?;
        let bid = right.parse()?;
        let cards: Vec<u8> = left
            .chars()
            .map(|c| {
                card_value(c).ok_or_else(|| AocError::Invalid(format!("invalid card in: {s}")))
            })
            .collect::<Result<_, _>>()?;
        let cards: [u8; 5] = cards
            .try_into()
            .map_err(|_| AocError::Invalid(format!("expected 5 cards: {s}")))?;
        Ok(Self { cards, bid })
    }
}

impl Hand {
    /// Packs the hand type into the 6th nibble and the five card values into the lower five.
    /// With `jokers`, J is wild: it contributes to the strongest group and ranks below 2.
    fn get_power(&self, jokers: bool) -> usize {
        let mut counts = [0u8; 15];
        let mut joker_count = 0u8;
        let mut power = 0usize;

        for (i, &c) in self.cards.iter().enumerate() {
            let is_joker = jokers && c == JOKER;
            if is_joker {
                joker_count += 1;
            } else {
                counts[c as usize] += 1;
            }
            let value = if is_joker { 1 } else { c };
            power |= (value as usize) << ((4 - i) * 4);
        }

        counts.sort_unstable_by(|a, b| b.cmp(a));
        if joker_count >= 4 {
            // Wilds alone can complete five of a kind.
            counts[0] = 5;
        } else {
            // Adding to the largest count keeps it the largest, so no re-sort is needed.
            counts[0] += joker_count;
        }

        power | (HandResult::get_hand_result(&counts) as usize) << 20
    }
}

struct InputData {
    hands: Vec<Hand>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            hands: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Sorts hands by power and returns the sum of bid × rank.
fn rank_hands(hands: &[Hand], jokers: bool) -> usize {
    let mut ranked: Vec<(usize, u32)> =
        hands.iter().map(|h| (h.get_power(jokers), h.bid)).collect();
    ranked.sort_unstable();
    ranked
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| *bid as usize * (rank + 1))
        .sum()
}

impl InputData {
    fn solve_part1(&self) -> usize {
        rank_hands(&self.hands, false)
    }

    fn solve_part2(&self) -> usize {
        rank_hands(&self.hands, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const TEST_CUSTOM_1: &str = "AAAAA 1
AAAAK 1
AAAKK 1
23456 1";

    /// The joker cases: a hand of nothing but jokers, five of a kind made by merging, and a full
    /// house made by merging.
    const TEST_CUSTOM_2: &str = "JJJJJ 1
JJJAA 10
AAJKK 100";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.hands[0].get_power(false), 0x232a3d);
        assert_eq!(solution_data.hands[1].get_power(false), 0x4a55b5);
        assert_eq!(solution_data.hands[2].get_power(false), 0x3dd677);
        assert_eq!(solution_data.hands[3].get_power(false), 0x3dabba);
        assert_eq!(solution_data.hands[4].get_power(false), 0x4cccbe);
        assert_eq!(solution_data.solve_part1(), 6440);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 123);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.hands[0].get_power(true), 0x232a3d);
        assert_eq!(solution_data.hands[1].get_power(true), 0x6a5515);
        assert_eq!(solution_data.hands[2].get_power(true), 0x3dd677);
        assert_eq!(solution_data.hands[3].get_power(true), 0x6da11a);
        assert_eq!(solution_data.hands[4].get_power(true), 0x6ccc1e);
        assert_eq!(solution_data.solve_part2(), 5905);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 10);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 132);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["234QK345", "234k5 12", "234567 2", "12345 3", "23045 10"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
    }

    #[test]
    fn parser_invalid_bid() {
        assert!(matches!(
            "AAAAA 4294967296".parse::<InputData>(),
            Err(AocError::ParseInt(_))
        ));
    }
}
