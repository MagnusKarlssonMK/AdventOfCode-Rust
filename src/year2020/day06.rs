//! # 2020 day 6 - Custom Customs
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Group {
    answers: Vec<String>,
}

impl FromStr for Group {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            answers: s.lines().map(str::to_string).collect(),
        })
    }
}

impl Group {
    fn get_yes_count_anyone(&self) -> usize {
        let mut yes: HashSet<char> = HashSet::from_iter(self.answers[0].chars());
        for a in self.answers.iter() {
            yes.extend(a.chars());
        }
        yes.len()
    }

    fn get_yes_count_everyone(&self) -> usize {
        let mut yes = self.answers[0].chars().collect::<HashSet<_>>();
        for a in self.answers.iter() {
            let no = a.chars().collect::<HashSet<_>>();
            yes = no.intersection(&yes).copied().collect();
        }
        yes.len()
    }
}

struct InputData {
    forms: Vec<Group>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            forms: s
                .split("\n\n")
                .map(|g| Group::from_str(g).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.forms.iter().map(|f| f.get_yes_count_anyone()).sum()
    }

    fn solve_part2(&self) -> usize {
        self.forms.iter().map(|f| f.get_yes_count_everyone()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "abcx\nabcy\nabcz";
    const TEST_DATA_2: &str = "abc\n
a\nb\nc\n
ab\nac\n
a\na\na\na\n
b";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 11);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }
}
