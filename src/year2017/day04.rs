use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData<'a> {
    passphrases: Vec<Vec<&'a str>>
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self { passphrases: input.lines()
            .map(|phrase| phrase.split_whitespace()
                .collect())
            .collect() }
    }

    fn solve_part1(&self) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        self.passphrases.iter()
            .filter(|phrase| {
                seen.clear();
                phrase.iter().all(|password| seen.insert(password.to_string()))
            })
            .count()
    }

    fn solve_part2(&self) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        self.passphrases.iter()
            .filter(|phrase| {
                let sorted_pwds: Vec<String> = phrase.iter()
                    .map(|password| {
                        let mut sorted_pwd: Vec<char> = password.chars().collect();
                        sorted_pwd.sort_unstable();
                        sorted_pwd.iter().collect()
                    }).collect();
                seen.clear();
                sorted_pwds.iter().all(|password| seen.insert(password.to_string()))
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("aa bb cc dd ee");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("aa bb cc dd aa");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("aa bb cc dd aaa");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("abcde fghij");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("abcde xyz ecdab");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from("a ab abc abd abf abj");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_4() {
        let testdata = String::from("iiii oiii ooii oooi oooo");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_5() {
        let testdata = String::from("oiii ioii iioi iiio");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 0);
    }
}
