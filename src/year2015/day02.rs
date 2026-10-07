//! # 2015 day 2 - I Was Told There Would Be No Math
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct InputData {
    gifts: Vec<(usize, usize, usize)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            gifts: s
                .lines()
                .map(|line| {
                    let mut sides: Vec<usize> = line
                        .split('x')
                        .map(|digit| digit.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()?;
                    sides.sort_unstable();
                    if sides.len() != 3 {
                        Err(AocError::Invalid(line.to_string()))
                    } else {
                        Ok((sides[0], sides[1], sides[2]))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.gifts
            .iter()
            .map(|(l, w, h)| 3 * (l * w) + 2 * h * (w + l))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.gifts
            .iter()
            .map(|(l, w, h)| 2 * (l + w) + (l * w * h))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "2x3x4";
    const TEST_OFFICIAL_2: &str = "1x1x10";
    const TEST_CUSTOM_1: &str = "4x3x2";
    const TEST_CUSTOM_2: &str = "3x4x5\n2x2x3";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 58);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 43);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 58);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 142);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 34);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 14);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 34);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 94);
    }

    #[test]
    fn parser_not_groups_of_three() {
        let err = InputData::from_str("1x2x4\n5x6").unwrap_err();
        assert_eq!(err, AocError::Invalid("5x6".to_string()));
    }

    #[test]
    fn parser_not_integer() {
        let err = InputData::from_str("1x2x4\n5xGx7").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
        //assert!(solve("1x2x4\n5xGx7").is_err());
    }
}
