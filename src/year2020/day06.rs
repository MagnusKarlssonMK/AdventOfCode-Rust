//! # 2020 day 6 - Custom Customs
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct Group {
    answers: Vec<String>,
}

impl Group {
    fn parse(input: &str) -> Self {
        Self {
            answers: input.lines().map(str::to_string).collect(),
        }
    }

    fn get_yes_count_anyone(&self) -> usize {
        let mut yes: HashSet<char> = HashSet::from_iter(self.answers[0].chars());
        for a in self.answers.iter() {
            yes.extend(a.chars());
        }
        yes.len()
    }

    fn get_yes_count_everyone(&self) -> usize {
        let mut yes = HashSet::from(self.answers[0].chars().collect::<HashSet<_>>());
        for a in self.answers.iter() {
            let no = HashSet::from(a.chars().collect::<HashSet<_>>());
            yes = no.intersection(&yes).copied().collect();
        }
        yes.len()
    }
}

struct InputData {
    forms: Vec<Group>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            forms: input.split("\n\n").map(Group::parse).collect(),
        }
    }

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

    #[test]
    fn part1_example_1() {
        let testdata = String::from("abcx\nabcy\nabcz");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from(
            "abc\n
a\nb\nc\n
ab\nac\n
a\na\na\na\n
b",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 11);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "abc\n
a\nb\nc\n
ab\nac\n
a\na\na\na\n
b",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 6);
    }
}
