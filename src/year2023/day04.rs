//! # 2023 day 4 - Scratchcards
use std::{collections::HashSet, error::Error, str::FromStr, vec};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Card {
    wincount: usize,
    score: usize,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, nbrs) = s.split_once(": ").unwrap();
        let (left, right) = nbrs.split_once(" | ").unwrap();
        let winning_nbrs = left.split_whitespace().map(|v| v.parse().unwrap());
        let winning_nbrs: HashSet<usize> = HashSet::from_iter(winning_nbrs);
        let draw_nbrs = right.split_whitespace().map(|v| v.parse().unwrap());
        let draw_nbrs: HashSet<usize> = HashSet::from_iter(draw_nbrs);
        let wincount = winning_nbrs.intersection(&draw_nbrs).count();
        let score = if wincount > 0 {
            2_usize.pow((wincount - 1).try_into().unwrap())
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            scratchcards: s.lines().map(|c| Card::from_str(c).unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.scratchcards.iter().map(|c| c.score).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut copylist = vec![1; self.scratchcards.len()];
        for (i, card) in self.scratchcards.iter().enumerate() {
            (1..=card.wincount).for_each(|j| copylist[i + j] += copylist[i]);
        }
        copylist.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 30);
    }
}
