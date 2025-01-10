//! # 2017 day 4 - High-Entropy Passphrases
use std::{collections::HashSet, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    passphrases: Vec<Vec<&'a str>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            passphrases: s
                .lines()
                .map(|phrase| phrase.split_whitespace().collect())
                .collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        self.passphrases
            .iter()
            .filter(|phrase| {
                seen.clear();
                phrase
                    .iter()
                    .all(|password| seen.insert(password.to_string()))
            })
            .count()
    }

    fn solve_part2(&self) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        self.passphrases
            .iter()
            .filter(|phrase| {
                let sorted_pwds: Vec<String> = phrase
                    .iter()
                    .map(|password| {
                        let mut sorted_pwd: Vec<char> = password.chars().collect();
                        sorted_pwd.sort_unstable();
                        sorted_pwd.iter().collect()
                    })
                    .collect();
                seen.clear();
                sorted_pwds
                    .iter()
                    .all(|password| seen.insert(password.to_string()))
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = "aa bb cc dd ee";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "aa bb cc dd aa";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "aa bb cc dd aaa";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "abcde fghij";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "abcde xyz ecdab";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "a ab abc abd abf abj";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_4() {
        let testdata = "iiii oiii ooii oooi oooo";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_5() {
        let testdata = "oiii ioii iioi iiio";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }
}
