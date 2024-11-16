use std::{collections::HashSet, vec};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
struct Card {
    wincount: usize,
    score: usize
}

impl Card {
    fn parse(input: &str) -> Self {
        let (_, nbrs) = input.split_once(": ").unwrap();
        let (left, right) = nbrs.split_once(" | ").unwrap();
        let winning_nbrs = left.split_whitespace().map(|v| v.parse().unwrap());
        let winning_nbrs: HashSet<usize> = HashSet::from_iter(winning_nbrs);
        let draw_nbrs = right.split_whitespace().map(|v| v.parse().unwrap());
        let draw_nbrs: HashSet<usize> = HashSet::from_iter(draw_nbrs);
        let wincount = winning_nbrs.intersection(&draw_nbrs).count();
        let score = if wincount > 0 { 2_usize.pow((wincount - 1).try_into().unwrap()) } else { 0 };
        Self {
            wincount,
            score
        }
    }
}

struct InputData {
    scratchcards: Vec<Card>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            scratchcards: input.lines().map(Card::parse).collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.scratchcards.iter().map(|c| c.score).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut copylist = vec![1; self.scratchcards.len()];
        for (i, card) in self.scratchcards.iter().enumerate() {
            (1..=card.wincount).for_each(|j| copylist[i+j] += copylist[i]);
        }
        copylist.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 30);
    }
}
