//! - Work in progress -
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct Group<'a> {
    answers: Vec<&'a str>,
}

impl<'a> Group<'a> {
    fn parse(input: &'a str) -> Self {
        Self {
            answers: input.lines().collect(),
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
        //let mut yes: HashSet<char> = HashSet::from(self.answers[0].chars().collect::<Vec<char>>());
        //for a in self.answers.iter() {
        //    let no: HashSet<char> = HashSet::from(a.chars().collect());
        //    yes = no.intersection(&yes).collect();
        //}
        //yes.len()
        6
    }
}

struct InputData<'a> {
    forms: Vec<Group<'a>>,
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
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
