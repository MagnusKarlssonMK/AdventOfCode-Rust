//! # 2020 day 18 - Operation Order
use std::{collections::VecDeque, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    lines: Vec<&'a str>
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { lines: s.lines().collect() })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        self.lines.iter().map(|line| shunting_yard(line, false)).sum()
    }

    fn solve_part2(&self) -> usize {
        self.lines.iter().map(|line| shunting_yard(line, true)).sum()
    }
}

fn shunting_yard(s: &str, is_advanced: bool) -> usize {
    let mut output = VecDeque::new();
    let mut opstack = VecDeque::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            output.push_back(c);
        } else {
            match c {
                '+' | '*' => {
                    while !opstack.is_empty() {
                        let &o = opstack.iter().last().unwrap();
                        if o != '(' && !(is_advanced && o == '*' && c == '+') {
                            output.push_back(opstack.pop_back().unwrap());
                        } else {
                            break;
                        }
                    }
                    opstack.push_back(c);
                },
                '(' => {
                    opstack.push_back(c);
                },
                ')' => {
                    while let Some(o) = opstack.pop_back() {
                        if o != '(' {
                            output.push_back(o);
                        } else {
                            break;
                        }
                    }
                },
                _ => (),
            }
        }
    }
    while let Some(o) = opstack.pop_back() {
        output.push_back(o);
    }

    let mut evaluated = VecDeque::new();
    for c in &output {
        match c {
            '+' | '*' => {
                let v1 = evaluated.pop_back().unwrap();
                let v2 = evaluated.pop_back().unwrap();
                let n = if *c == '+' { v1 + v2 } else { v1 * v2 };
                evaluated.push_back(n);
            },
            _ => {
                evaluated.push_back(c.to_digit(10).unwrap() as usize);
            },
        }
    }
    *evaluated.front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const TEST_DATA_2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const TEST_DATA_3: &str = "2 * 3 + (4 * 5)";
    const TEST_DATA_4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const TEST_DATA_5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const TEST_DATA_6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::try_from(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 71);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::try_from(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 51);
    }

    #[test]
    fn part1_example_3() {
        let solution_data = InputData::try_from(TEST_DATA_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 26);
    }

    #[test]
    fn part1_example_4() {
        let solution_data = InputData::try_from(TEST_DATA_4).unwrap();
        assert_eq!(solution_data.solve_part1(), 437);
    }

    #[test]
    fn part1_example_5() {
        let solution_data = InputData::try_from(TEST_DATA_5).unwrap();
        assert_eq!(solution_data.solve_part1(), 12240);
    }

    #[test]
    fn part1_example_6() {
        let solution_data = InputData::try_from(TEST_DATA_6).unwrap();
        assert_eq!(solution_data.solve_part1(), 13632);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 231);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::try_from(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 51);
    }

    #[test]
    fn part2_example_3() {
        let solution_data = InputData::try_from(TEST_DATA_3).unwrap();
        assert_eq!(solution_data.solve_part2(), 46);
    }

    #[test]
    fn part2_example_4() {
        let solution_data = InputData::try_from(TEST_DATA_4).unwrap();
        assert_eq!(solution_data.solve_part2(), 1445);
    }

    #[test]
    fn part2_example_5() {
        let solution_data = InputData::try_from(TEST_DATA_5).unwrap();
        assert_eq!(solution_data.solve_part2(), 669060);
    }

    #[test]
    fn part2_example_6() {
        let solution_data = InputData::try_from(TEST_DATA_6).unwrap();
        assert_eq!(solution_data.solve_part2(), 23340);
    }
}
