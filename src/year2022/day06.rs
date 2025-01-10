//! # 2022 day 6 - Tuning Trouble
//!
//! Use the 'windows' feature of Rust to scan the input, and throw every
//! such slice into a hashset to see if the length equals the slice length.
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    datastream: Vec<char>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            datastream: (s.chars().collect()),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.get_processed_characters(4).unwrap()
    }

    fn solve_part2(&self) -> usize {
        self.get_processed_characters(14).unwrap()
    }

    fn get_processed_characters(&self, start: usize) -> Option<usize> {
        let mut set: HashSet<char> = HashSet::new();
        for (i, part) in self.datastream.windows(start).enumerate() {
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
        let testdata = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
        assert_eq!(solution_data.solve_part2(), 19);
    }

    #[test]
    fn part1_2_example_2() {
        let testdata = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
        assert_eq!(solution_data.solve_part2(), 23);
    }

    #[test]
    fn part1_2_example_3() {
        let testdata = "nppdvjthqldpwncqszvftbrmjlhg";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
        assert_eq!(solution_data.solve_part2(), 23);
    }

    #[test]
    fn part1_2_example_4() {
        let testdata = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
        assert_eq!(solution_data.solve_part2(), 29);
    }

    #[test]
    fn part1_2_example_5() {
        let testdata = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 11);
        assert_eq!(solution_data.solve_part2(), 26);
    }
}
