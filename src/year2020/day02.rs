//! # 2020 day 2 - Password Philosophy
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Password {
    nbr_range: (usize, usize),
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let (r1, r2) = words.next().unwrap().split_once('-').unwrap();
        Ok(Self {
            nbr_range: (r1.parse().unwrap(), r2.parse().unwrap()),
            letter: words.next().unwrap().trim_matches(':').parse().unwrap(),
            password: words.next().unwrap().parse().unwrap(),
        })
    }
}

impl Password {
    fn is_valid_old_policy(&self) -> bool {
        (self.nbr_range.0..=self.nbr_range.1)
            .contains(&self.password.chars().filter(|c| *c == self.letter).count())
    }

    fn is_valid_new_policy(&self) -> bool {
        if self.password.len() < self.nbr_range.0 {
            false
        } else {
            let first = self.letter == self.password.chars().nth(self.nbr_range.0 - 1).unwrap();
            let second = self.password.len() >= self.nbr_range.1
                && self.letter == self.password.chars().nth(self.nbr_range.1 - 1).unwrap();
            first ^ second
        }
    }
}

struct InputData {
    passwords: Vec<Password>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            passwords: s
                .lines()
                .map(|line| Password::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.passwords
            .iter()
            .filter(|p| p.is_valid_old_policy())
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.passwords
            .iter()
            .filter(|p| p.is_valid_new_policy())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }
}
