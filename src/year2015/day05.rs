//! # 2015 day 5 - Doesn't He Have Intern-Elves For This?
use crate::aoc_util::error::AocError;
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    santa_strings: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            santa_strings: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        fn is_nice(word: &str) -> bool {
            let mut previous_char: Option<char> = None;
            let mut vowel_counter = 0;
            let mut double_char = false;
            for c in word.chars() {
                if VOWELS.contains(&c) {
                    vowel_counter += 1;
                }
                if let Some(prev) = previous_char {
                    if (prev == 'a' && c == 'b')
                        || (prev == 'c' && c == 'd')
                        || (prev == 'p' && c == 'q')
                        || (prev == 'x' && c == 'y')
                    {
                        return false;
                    }
                    if c == prev {
                        double_char = true;
                    }
                }
                previous_char = Some(c);
            }
            vowel_counter >= 3 && double_char
        }
        self.santa_strings
            .iter()
            .filter(|word| is_nice(word))
            .count()
    }

    fn solve_part2(&self) -> usize {
        fn is_nice(word: &str) -> bool {
            if word
                .chars()
                .zip(word.chars().skip(2))
                .any(|(c1, c2)| c1 == c2)
            {
                for idx in 0..word.len() - 1 {
                    let candidate = &word[idx..idx + 2];
                    if word[..idx].contains(candidate) || word[idx + 2..].contains(candidate) {
                        return true;
                    }
                }
            }
            false
        }
        self.santa_strings
            .iter()
            .filter(|word| is_nice(word))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

    const TEST_OFFICIAL_2: &str = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

    const TEST_CUSTOM_1: &str = "aaab
aeecd
aaeipq
aeiouxxyy";

    const TEST_CUSTOM_2: &str = "abab
bcbc
abcdab";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
