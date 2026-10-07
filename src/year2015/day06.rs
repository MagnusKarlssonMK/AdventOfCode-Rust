//! # 2015 day 6 - Probably a Fire Hazard
use crate::aoc_util::error::AocError;
use std::{
    cmp::{max, min},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Operation {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(6) {
            Some('n') => Ok(Self::TurnOn),
            Some('f') => Ok(Self::TurnOff),
            Some(' ') => Ok(Self::Toggle),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Area {
    x_range: (usize, usize),
    y_range: (usize, usize),
}

impl FromStr for Area {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nbrs: Vec<usize> = s.split([' ', ',']).filter_map(|s| s.parse().ok()).collect();
        if nbrs.len() != 4 {
            Err(AocError::Invalid(s.to_string()))
        } else {
            Ok(Self {
                x_range: (min(nbrs[0], nbrs[2]), max(nbrs[0], nbrs[2])),
                y_range: (min(nbrs[1], nbrs[3]), max(nbrs[1], nbrs[3])),
            })
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    area: Area,
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            op: Operation::from_str(line)?,
            area: Area::from_str(line)?,
        })
    }
}

const GRIDSIZE: usize = 1000;

#[derive(Debug)]
struct InputData {
    santa_instructions: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            santa_instructions: s
                .lines()
                .map(Instruction::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut grid = vec![vec![false; GRIDSIZE]; GRIDSIZE];
        for instr in &self.santa_instructions {
            for x in instr.area.x_range.0..=instr.area.x_range.1 {
                for g_y in grid
                    .iter_mut()
                    .take(instr.area.y_range.1 + 1)
                    .skip(instr.area.y_range.0)
                {
                    g_y[x] = match instr.op {
                        Operation::TurnOff => false,
                        Operation::TurnOn => true,
                        Operation::Toggle => !g_y[x],
                    }
                }
            }
        }
        grid.iter()
            .map(|y| y.iter().filter(|&&state| state).count())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut grid = vec![vec![0; GRIDSIZE]; GRIDSIZE];
        for instr in &self.santa_instructions {
            for x in instr.area.x_range.0..=instr.area.x_range.1 {
                for g_y in grid
                    .iter_mut()
                    .take(instr.area.y_range.1 + 1)
                    .skip(instr.area.y_range.0)
                {
                    g_y[x] = match instr.op {
                        Operation::TurnOff => {
                            if g_y[x] > 0 {
                                g_y[x] - 1
                            } else {
                                0
                            }
                        }
                        Operation::TurnOn => g_y[x] + 1,
                        Operation::Toggle => g_y[x] + 2,
                    }
                }
            }
        }
        grid.iter().map(|y| y.iter().sum::<usize>()).sum()
    }
}

#[cfg(test)]
mod tests {
    const TEST_OFFICIAL_1: &str = "turn on 0,0 through 999,999";
    const TEST_OFFICIAL_2: &str = "toggle 0,0 through 999,0";
    const TEST_OFFICIAL_3: &str = "turn on 0,0 through 999,999\nturn off 499,499 through 500,500";
    const TEST_OFFICIAL_4: &str = "turn on 0,0 through 0,0";
    const TEST_OFFICIAL_5: &str = "toggle 0,0 through 999,999";
    const TEST_CUSTOM_1: &str = "toggle 0,0 through 999,999\nturn on 0,0 through 999,999";
    const TEST_CUSTOM_2: &str = "turn off 0,0 through 999,999";
    const TEST_CUSTOM_3: &str = "toggle 0,0 through 999,999\ntoggle 0,0 through 999,999";
    const TEST_CUSTOM_4: &str =
        "turn on 0,0 through 999,999\nturn on 0,0 through 999,999\nturn off 0,0 through 999,999";
    const TEST_CUSTOM_5: &str = "turn on 0,5 through 999,5";

    use super::*;
    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 1000 * 1000);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 1000);
    }

    #[test]
    fn part1_official_3() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 1000 * 1000 - 4);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 1000000);
    }

    #[test]
    fn part1_custom_3() {
        let solution_data = InputData::from_str(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_custom_5() {
        let solution_data = InputData::from_str(TEST_CUSTOM_5).unwrap();
        assert_eq!(solution_data.solve_part1(), 1000);
    }

    #[test]
    fn part2_official_4() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_4).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_5() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_5).unwrap();
        assert_eq!(solution_data.solve_part2(), 2000000);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 3000000);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_custom_3() {
        let solution_data = InputData::from_str(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part2(), 4000000);
    }

    #[test]
    fn part2_custom_4() {
        let solution_data = InputData::from_str(TEST_CUSTOM_4).unwrap();
        assert_eq!(solution_data.solve_part2(), 1000000);
    }

    #[test]
    fn parser_unknown_operation() {
        let s1 = "turn middle 0,0 through 999,999";
        let err1 = InputData::from_str(s1).unwrap_err();
        assert_eq!(err1, AocError::Invalid(s1.to_string()));
        let err2 = InputData::from_str("turn").unwrap_err();
        assert_eq!(err2, AocError::Invalid("turn".to_string()));
    }

    #[test]
    fn parser_invalid_coordinates() {
        let s1 = "turn middle 0,0 through 999999";
        let err1 = InputData::from_str(s1).unwrap_err();
        assert_eq!(err1, AocError::Invalid(s1.to_string()));
    }
}
