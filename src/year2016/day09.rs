//! # 2016 day 9 - Explosives in Cyberspace
use crate::aoc_util::error::{AocError, OptionExt};
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData<'a> {
    filedata: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { filedata: s })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> Result<usize, AocError> {
        get_decompressed_len(self.filedata, false)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        get_decompressed_len(self.filedata, true)
    }
}

fn get_decompressed_len(filedata: &str, recurse: bool) -> Result<usize, AocError> {
    let mut fp = 0;
    let mut total = 0;
    while let Some(c) = filedata.chars().nth(fp) {
        match c {
            '(' => {
                fp += 1;
                let mut marker = Vec::new();
                while let Some(d) = filedata.chars().nth(fp) {
                    if d != ')' {
                        marker.push(d);
                        fp += 1;
                    } else {
                        break;
                    }
                }
                fp += 1;
                let m = marker.iter().collect::<String>();
                let (a, b) = m.split_once('x').ctx("x separator")?;
                let a: usize = a.parse()?;
                let b: usize = b.parse()?;
                if recurse {
                    let block = filedata
                        .get(fp..fp + a)
                        .ctx_invalid(format!("marker {m} exceeds input range"))?;
                    total += b * get_decompressed_len(block, true)?;
                } else {
                    total += a * b;
                }
                fp += a;
            }
            _ => {
                fp += 1;
                total += 1;
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let testdata = "ADVENT";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 6);
    }

    #[test]
    fn part1_official_2() {
        let testdata = "A(1x5)BC";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 7);
    }

    #[test]
    fn part1_official_3() {
        let testdata = "(3x3)XYZ";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 9);
    }

    #[test]
    fn part1_official_4() {
        let testdata = "A(2x2)BCD(2x2)EFG";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 11);
    }

    #[test]
    fn part1_official_5() {
        let testdata = "(6x1)(1x3)A";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 6);
    }

    #[test]
    fn part1_official_6() {
        let testdata = "X(8x2)(3x3)ABCY";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 18);
    }

    #[test]
    fn part2_official_1() {
        let testdata = "(3x3)XYZ";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 9);
    }

    #[test]
    fn part2_official_2() {
        let testdata = "X(8x2)(3x3)ABCY";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 20);
    }

    #[test]
    fn part2_official_3() {
        let testdata = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 241920);
    }

    #[test]
    fn part2_official_4() {
        let testdata = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 445);
    }

    #[test]
    fn part1_missing_x() {
        let testdata = "(33)";
        let solution_data = InputData::try_from(testdata).unwrap();
        let err = solution_data.solve_part1().unwrap_err();
        assert_eq!(err, AocError::Missing("x separator"));
    }

    #[test]
    fn part1_parse_int() {
        let testdata = "(3x2f3)";
        let solution_data = InputData::try_from(testdata).unwrap();
        let err = solution_data.solve_part1().unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn part2_out_of_bounds() {
        let testdata = "(25x3)(3x2)";
        let solution_data = InputData::try_from(testdata).unwrap();
        let err = solution_data.solve_part2().unwrap_err();
        assert_eq!(
            err,
            AocError::Invalid("marker 25x3 exceeds input range".to_string())
        );
    }
}
