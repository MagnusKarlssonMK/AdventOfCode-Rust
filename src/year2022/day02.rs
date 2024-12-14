pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn get_val(&self) -> usize {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissors => 2,
        }
    }

    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
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

impl Guide {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }

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

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            rounds: input
                .lines()
                .map(|line| {
                    let hands: Vec<&str> = line.split_whitespace().collect();
                    (Hand::from(hands[0]), Guide::from(hands[1]))
                })
                .collect(),
        }
    }

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

    #[test]
    fn part1_example_1() {
        let testdata = String::from("A Y\nB X\nC Z");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 15);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("A Y\nB X\nC Z");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 12);
    }
}
