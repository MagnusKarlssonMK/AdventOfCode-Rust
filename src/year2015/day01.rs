//! # 2015 day 1 - Not Quite Lisp
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    directions: Vec<isize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            directions: s
                .chars()
                .map(|c| match &c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> isize {
        self.directions.iter().sum()
    }

    fn solve_part2(&self) -> isize {
        let mut floor = 0;
        let mut steps = 0;
        for v in &self.directions {
            floor += v;
            steps += 1;
            if floor < 0 {
                break;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = "(())";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "()()";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "(((";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "(()(()(";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_5() {
        let testdata = "))(((((";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_6() {
        let testdata = "())";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), -1);
    }

    #[test]
    fn part1_example_7() {
        let testdata = "))(";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), -1);
    }

    #[test]
    fn part1_example_8() {
        let testdata = ")))";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), -3);
    }

    #[test]
    fn part1_example_9() {
        let testdata = ")())())";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), -3);
    }

    #[test]
    fn part2_example_1() {
        let testdata = ")";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "()())";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 5);
    }
}
