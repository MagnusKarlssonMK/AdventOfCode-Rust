//! # 2015 day 19 - Medicine for Rudolph
use crate::aoc_util::error::{AocError, OptionExt};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct InputData<'a> {
    replacements: HashMap<String, Vec<String>>,
    molecule: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (r, molecule) = s.split_once("\n\n").ctx("missing newline separator")?;
        let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
        for line in r.lines() {
            let (left, right) = line.split_once(" => ").ctx("missing =>")?;
            replacements
                .entry(left.to_string())
                .and_modify(|v| v.push(right.to_string()))
                .or_insert(vec![right.to_string()]);
        }
        Ok(Self {
            replacements,
            molecule,
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        let mut altered: HashSet<String> = HashSet::new();
        for (replaced, v) in &self.replacements {
            for replacement in v.iter() {
                for (i, _) in self.molecule.match_indices(replaced) {
                    let j = i + replaced.len();
                    altered
                        .insert(self.molecule[..i].to_string() + replacement + &self.molecule[j..]);
                }
            }
        }
        altered.len()
    }

    fn solve_part2(&self) -> usize {
        let elements = self
            .molecule
            .chars()
            .filter(char::is_ascii_uppercase)
            .count();
        let rn = self.molecule.matches("Rn").count();
        let ar = self.molecule.matches("Ar").count();
        let y = self.molecule.matches('Y').count();
        elements - ar - rn - 2 * y - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "H => HO
H => OH
O => HH

HOH";

    const TEST_OFFICIAL_2: &str = "H => HO
H => OH
O => HH

HOHOHO";

    const TEST_CUSTOM_1: &str = "e => HF
F => CaF
Ca => CaCa
H => CRnMgYFAr
Ca => SiRnMgAr
Mg => TiMg

CRnMgYCaFArSiRnMgArCaCaF";

    const TEST_CUSTOM_2: &str = "e => HF
F => CaF
Ca => CaCa
H => CRnMgYFAr
Ca => SiRnMgAr
Mg => TiMg

HCaSiRnTiMgArCaF";

    const TEST_CUSTOM_3: &str = "e => HF
F => CaF
Ca => CaCa
H => CRnMgYFAr
Ca => SiRnMgAr
Mg => TiMg

HCaCaCaCaF";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part1_custom_3() {
        let solution_data = InputData::try_from(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 7);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn part2_custom_3() {
        let solution_data = InputData::try_from(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part2(), 5);
    }

    #[test]
    fn parse_invalid_input() {
        let err1 = InputData::try_from("F => CaF").unwrap_err();
        assert_eq!(err1, AocError::Missing("missing newline separator"));

        let err2 = InputData::try_from("F > CaF\n\nAbCD").unwrap_err();
        assert_eq!(err2, AocError::Missing("missing =>"));
    }
}
