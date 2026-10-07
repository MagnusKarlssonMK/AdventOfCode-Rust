//! # 2015 day 10 - Elves Look, Elves Say
//!
//! Straight transfer from my python solution => brute force
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
    start_numbers: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { start_numbers: s })
    }
}

impl<'a> InputData<'a> {
    fn get_generated_length(&self, rounds: usize) -> usize {
        let mut sequence = self.start_numbers.to_string();
        for _ in 0..rounds {
            let mut new_seq = String::new();
            let mut count = 0;
            let mut currentchar = ' ';
            for c in sequence.chars() {
                if c == currentchar {
                    count += 1;
                } else {
                    if count > 0 {
                        new_seq += &(count.to_string() + &currentchar.to_string());
                    }
                    count = 1;
                    currentchar = c;
                }
            }
            if count > 0 {
                new_seq += &(count.to_string() + &currentchar.to_string());
            }
            sequence = new_seq;
        }
        sequence.len()
    }

    fn solve_part1(&self) -> usize {
        self.get_generated_length(40)
    }

    fn solve_part2(&self) -> usize {
        self.get_generated_length(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from("1").unwrap();
        assert_eq!(solution_data.get_generated_length(1), 2);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::try_from("11").unwrap();
        assert_eq!(solution_data.get_generated_length(1), 2);
    }

    #[test]
    fn part1_official_3() {
        let solution_data = InputData::try_from("21").unwrap();
        assert_eq!(solution_data.get_generated_length(1), 4);
    }

    #[test]
    fn part1_official_4() {
        let solution_data = InputData::try_from("1211").unwrap();
        assert_eq!(solution_data.get_generated_length(1), 6);
    }

    #[test]
    fn part1_official_5() {
        let solution_data = InputData::try_from("111221").unwrap();
        assert_eq!(solution_data.get_generated_length(1), 6);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::try_from("22").unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::try_from("1").unwrap();
        assert_eq!(solution_data.solve_part1(), 82350);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::try_from("22").unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::try_from("1").unwrap();
        assert_eq!(solution_data.solve_part2(), 1166642);
    }
}
