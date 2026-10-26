//! # 2017 day 4 - High-Entropy Passphrases
//!
//! Split each passphrase into words. Part 1 counts passphrases where all words
//! are unique. Part 2 counts passphrases where no word is an anagram of
//! another, which is checked by sorting the letters of each word.
use std::{collections::HashSet, error::Error};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Passphrase<'a> {
    words: Vec<&'a str>,
}

impl<'a> Passphrase<'a> {
    fn is_valid_simple(&self) -> bool {
        let mut seen: HashSet<String> = HashSet::new();
        self.words.iter().all(|word| seen.insert(word.to_string()))
    }

    fn is_valid_advanced(&self) -> bool {
        let mut seen: HashSet<String> = HashSet::new();
        self.words.iter().all(|word| {
            let mut sorted: Vec<char> = word.chars().collect();
            sorted.sort_unstable();
            seen.insert(sorted.iter().collect())
        })
    }
}

struct InputData<'a> {
    passphrases: Vec<Passphrase<'a>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            passphrases: s
                .lines()
                .map(|phrase| Passphrase {
                    words: phrase.split_whitespace().collect(),
                })
                .collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        self.passphrases
            .iter()
            .filter(|phrase| phrase.is_valid_simple())
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.passphrases
            .iter()
            .filter(|phrase| phrase.is_valid_advanced())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from("aa bb cc dd ee").unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::try_from("aa bb cc dd aa").unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_official_3() {
        let solution_data = InputData::try_from("aa bb cc dd aaa").unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from("abcde fghij").unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from("abcde xyz ecdab").unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_official_3() {
        let solution_data = InputData::try_from("a ab abc abd abf abj").unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_4() {
        let solution_data = InputData::try_from("iiii oiii ooii oooi oooo").unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_5() {
        let solution_data = InputData::try_from("oiii ioii iioi iiio").unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::try_from("abc cba").unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data = InputData::try_from("aa aa").unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 0);
    }
}
