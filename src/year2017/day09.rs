//! # 2017 day 9 - Stream Processing
use std::error::Error;

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    let (p1, p2) = solution_data.solve_parts_1_2();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData<'a> {
    stream: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { stream: s })
    }
}

impl InputData<'_> {
    fn solve_parts_1_2(&self) -> (usize, usize) {
        let mut tokens = self.stream.chars();
        let mut garbage_count = 0;
        let mut score = 0;
        let mut level = 1;
        while let Some(token) = tokens.next() {
            match token {
                '<' => {
                    while let Some(garbage) = tokens.next() {
                        match garbage {
                            '!' => {
                                tokens.next();
                            }
                            '>' => break,
                            _ => garbage_count += 1,
                        }
                    }
                }
                '{' => {
                    score += level;
                    level += 1;
                }
                '}' => {
                    level -= 1;
                }
                _ => (),
            }
        }
        (score, garbage_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_official_1() {
        let solution_data = InputData::try_from("{}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 1);
    }

    #[test]
    fn part_1_official_2() {
        let solution_data = InputData::try_from("{{{}}}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 6);
    }

    #[test]
    fn part_1_official_3() {
        let solution_data = InputData::try_from("{{},{}}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 5);
    }

    #[test]
    fn part_1_official_4() {
        let solution_data = InputData::try_from("{{{},{},{{}}}}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 16);
    }

    #[test]
    fn part_1_official_5() {
        let solution_data = InputData::try_from("{<a>,<a>,<a>,<a>}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 1);
    }

    #[test]
    fn part_1_official_6() {
        let solution_data = InputData::try_from("{{<ab>},{<ab>},{<ab>},{<ab>}}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 9);
    }

    #[test]
    fn part_1_official_7() {
        let solution_data = InputData::try_from("{{<!!>},{<!!>},{<!!>},{<!!>}}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 9);
    }

    #[test]
    fn part_1_official_8() {
        let solution_data = InputData::try_from("{{<a!>},{<a!>},{<a!>},{<ab>}}").unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 3);
    }

    #[test]
    fn part_2_official_1() {
        let solution_data = InputData::try_from("<>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_official_2() {
        let solution_data = InputData::try_from("<random characters>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 17);
    }

    #[test]
    fn part_2_official_3() {
        let solution_data = InputData::try_from("<<<<>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 3);
    }

    #[test]
    fn part_2_official_4() {
        let solution_data = InputData::try_from("<{!>}>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 2);
    }

    #[test]
    fn part_2_official_5() {
        let solution_data = InputData::try_from("<!!>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_official_6() {
        let solution_data = InputData::try_from("<!!!>>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_official_7() {
        let solution_data = InputData::try_from("<{o\"i!a,<{i<a>").unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 10);
    }
}
