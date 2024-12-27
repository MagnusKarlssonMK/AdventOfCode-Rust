//! # 2022 day 6 - Tuning Trouble
//!
//! Use the 'windows' feature of Rust to scan the input, and throw every
//! such slice into a hashset to see if the length equals the slice length.
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    data: Vec<char>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            data: input.chars().collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        self.get_processed_characters(4).unwrap()
    }

    fn solve_part2(&self) -> usize {
        self.get_processed_characters(14).unwrap()
    }

    fn get_processed_characters(&self, start: usize) -> Option<usize> {
        let mut set: HashSet<char> = HashSet::new();
        for (i, part) in self.data.windows(start).enumerate() {
            set.extend(part);
            if set.len() == start {
                return Some(i + start);
            }
            set.clear();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_example_1() {
        let testdata = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 7);
        assert_eq!(solution_data.solve_part2(), 19);
    }

    #[test]
    fn part1_2_example_2() {
        let testdata = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 5);
        assert_eq!(solution_data.solve_part2(), 23);
    }

    #[test]
    fn part1_2_example_3() {
        let testdata = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 6);
        assert_eq!(solution_data.solve_part2(), 23);
    }

    #[test]
    fn part1_2_example_4() {
        let testdata = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 10);
        assert_eq!(solution_data.solve_part2(), 29);
    }

    #[test]
    fn part1_2_example_5() {
        let testdata = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 11);
        assert_eq!(solution_data.solve_part2(), 26);
    }
}
