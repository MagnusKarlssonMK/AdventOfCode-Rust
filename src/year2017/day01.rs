//! # 2017 day 1 - Inverse Captcha
use crate::aoc_util::error::AocError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct InputData {
    sequence: Vec<u32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sequence: s
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| AocError::Invalid(format!("not a number: {c}")))
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn get_captcha(&self, halfway: bool) -> u32 {
        let len = self.sequence.len();
        let v_offset = if !halfway { 1 } else { len / 2 };
        self.sequence
            .iter()
            .enumerate()
            .filter(|(i, nbr)| self.sequence[(i + v_offset) % len] == **nbr)
            .map(|(_, &nbr)| nbr)
            .sum::<u32>()
    }

    fn solve_part1(&self) -> u32 {
        self.get_captcha(false)
    }

    fn solve_part2(&self) -> u32 {
        self.get_captcha(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_official_1() {
        let solution_data: InputData = "1122".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = "1111".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = "1234".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_official_4() {
        let solution_data: InputData = "91212129".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 9);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = "1212".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = "1221".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = "123425".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }

    #[test]
    fn part2_official_4() {
        let solution_data: InputData = "123123".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 12);
    }

    #[test]
    fn part2_official_5() {
        let solution_data: InputData = "12131415".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }

    #[test]
    fn parser_non_integer() {
        let err = "123f24".parse::<InputData>().unwrap_err();
        assert_eq!(err, AocError::Invalid("not a number: f".to_string()));
    }
}
