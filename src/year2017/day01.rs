//! # 2017 day 1 - Inverse Captcha
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    sequence: Vec<u8>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sequence: s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
        })
    }
}

impl InputData {
    fn get_captcha(&self, halfway: bool) -> usize {
        let v_offset = if !halfway { 1 } else { self.sequence.len() / 2 };
        self.sequence
            .iter()
            .enumerate()
            .filter(|(i, nbr)| {
                self.sequence
                    .get((i + v_offset) % self.sequence.len())
                    .unwrap()
                    == *nbr
            })
            .map(|(_, &nbr)| nbr as usize)
            .sum::<usize>()
    }

    fn solve_part1(&self) -> usize {
        self.get_captcha(false)
    }

    fn solve_part2(&self) -> usize {
        self.get_captcha(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = "1122";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "1111";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "1234";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "91212129";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 9);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "1212";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "1221";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "123425";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }

    #[test]
    fn part2_example_4() {
        let testdata = "123123";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 12);
    }

    #[test]
    fn part2_example_5() {
        let testdata = "12131415";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
