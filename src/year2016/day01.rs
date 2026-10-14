//! # 2016 day 1 - No Time for a Taxicab
use crate::aoc_util::{error::AocError, point::*};
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(Debug)]
enum Instruction {
    RotateLeft(u32),
    RotateRight(u32),
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(AocError::Invalid(s.to_string()));
        }
        let steps = s[1..].parse::<u32>()?;
        match s.chars().nth(0) {
            Some('L') => Ok(Self::RotateLeft(steps)),
            Some('R') => Ok(Self::RotateRight(steps)),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Instruction {
    fn apply(&self, direction: &mut Point) -> u32 {
        match self {
            Instruction::RotateLeft(steps) => {
                *direction = direction.rotate_left();
                *steps
            }
            Instruction::RotateRight(steps) => {
                *direction = direction.rotate_right();
                *steps
            }
        }
    }
}

#[derive(Debug)]
struct InputData {
    instructions: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: s
                .split(", ")
                .map(Instruction::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut pos = ORIGIN;
        let mut direction = UP;
        for instr in &self.instructions {
            let steps = instr.apply(&mut direction);
            pos += direction * steps as i32;
        }
        pos.manhattan(&ORIGIN)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let mut pos = ORIGIN;
        let mut direction = UP;
        let mut seen: HashSet<Point> = HashSet::new();
        seen.insert(pos);
        for instr in &self.instructions {
            let steps = instr.apply(&mut direction);
            for _ in 0..steps {
                pos += direction;
                if seen.contains(&pos) {
                    return Ok(pos.manhattan(&ORIGIN));
                }
                seen.insert(pos);
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_official_1() {
        let testdata = "R2, L3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part1_official_2() {
        let testdata = "R2, R2, R2";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_official_3() {
        let testdata = "R5, L5, R5, R3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part2_official_1() {
        let testdata = "R8, R4, R4, R8";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(4));
    }

    #[test]
    fn part2_custom_1() {
        let testdata = "R1, L1, L1, L1, R3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(0));
    }

    #[test]
    fn part2_custom_2() {
        let testdata = "L1, L1, R1, R1, R1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(1));
    }

    #[test]
    fn parser_invalid_int() {
        let testdata = "R32, LT4, L8";
        let err = InputData::from_str(testdata).unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn parser_too_short() {
        let testdata = "R32, L, L8";
        let err = InputData::from_str(testdata).unwrap_err();
        assert_eq!(err, AocError::Invalid("L".to_string()));
    }

    #[test]
    fn parser_invalid_direction() {
        let testdata = "R32, K4, L8";
        let err = InputData::from_str(testdata).unwrap_err();
        assert_eq!(err, AocError::Invalid("K4".to_string()));
    }

    #[test]
    fn parser_negative_int() {
        let testdata = "R32, L-4, L8";
        let err = InputData::from_str(testdata).unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn part2_no_solution() {
        let testdata = "R2, L3";
        let solution_data = InputData::from_str(testdata).unwrap();
        let err = solution_data.solve_part2().unwrap_err();
        assert_eq!(err, AocError::NoSolution);
    }
}
