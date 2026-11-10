//! # 2020 day 18 - Operation Order
//!
//! Uses the shunting yard algorithm to solve the problem.
//!
//! Evaluate each expression with the shunting yard algorithm. Part 1 gives both operators equal
//! precedence (plain left to right); part 2 gives "+" higher precedence than "*". The only
//! difference is the precedence function used by the evaluator.
use crate::aoc_util::error::AocError;
use std::{collections::VecDeque, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData<'a> {
    lines: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            lines: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn evaluate(&self, is_advanced: bool) -> Result<u64, AocError> {
        self.lines
            .iter()
            .map(|line| shunting_yard(line, is_advanced))
            .try_fold(0u64, |total, line_result| {
                total.checked_add(line_result?).ok_or(AocError::NoSolution)
            })
    }

    fn solve_part1(&self) -> Result<u64, AocError> {
        self.evaluate(false)
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        self.evaluate(true)
    }
}

fn shunting_yard(s: &str, is_advanced: bool) -> Result<u64, AocError> {
    fn precedence(op: char, is_advanced: bool) -> u8 {
        match op {
            '+' if is_advanced => 2,
            '+' | '*' => 1,
            _ => 0,
        }
    }
    let mut output = VecDeque::new();
    let mut opstack = VecDeque::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            output.push_back(c);
        } else {
            match c {
                '+' | '*' => {
                    while let Some(o) = opstack.pop_back() {
                        if o != '(' && precedence(o, is_advanced) >= precedence(c, is_advanced) {
                            output.push_back(o);
                        } else {
                            opstack.push_back(o);
                            break;
                        }
                    }
                    opstack.push_back(c);
                }
                '(' => {
                    opstack.push_back(c);
                }
                ')' => {
                    let mut found_open = false;
                    while let Some(o) = opstack.pop_back() {
                        if o == '(' {
                            found_open = true;
                            break;
                        }
                        output.push_back(o);
                    }
                    if !found_open {
                        return Err(AocError::Invalid("unmatched ')'".to_string()));
                    }
                }
                ' ' => (),
                _ => return Err(AocError::Invalid(c.to_string())),
            }
        }
    }
    while let Some(o) = opstack.pop_back() {
        if o == '(' {
            return Err(AocError::Invalid("unmatched '('".to_string()));
        }
        output.push_back(o);
    }

    let mut evaluated = VecDeque::new();
    for c in &output {
        match c {
            '+' | '*' => {
                let v1: u64 = evaluated
                    .pop_back()
                    .ok_or_else(|| AocError::Invalid(format!("missing operand for '{c}'")))?;
                let v2: u64 = evaluated
                    .pop_back()
                    .ok_or_else(|| AocError::Invalid(format!("missing operand for '{c}'")))?;
                let n = if *c == '+' {
                    v1.checked_add(v2)
                } else {
                    v1.checked_mul(v2)
                }
                .ok_or(AocError::NoSolution)?;
                evaluated.push_back(n);
            }
            _ => {
                evaluated.push_back(
                    c.to_digit(10)
                        .ok_or_else(|| AocError::Invalid(format!("not a digit: {c}")))?
                        as u64,
                );
            }
        }
    }
    let result = evaluated
        .pop_back()
        .ok_or(AocError::Invalid("empty expression".to_string()))?;
    if !evaluated.is_empty() {
        Err(AocError::Invalid("unexpected extra operands".to_string()))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const TEST_OFFICIAL_2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const TEST_OFFICIAL_3: &str = "2 * 3 + (4 * 5)";
    const TEST_OFFICIAL_4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const TEST_OFFICIAL_5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const TEST_OFFICIAL_6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 71);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 51);
    }

    #[test]
    fn part1_official_3() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_3).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 26);
    }

    #[test]
    fn part1_official_4() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_4).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 437);
    }

    #[test]
    fn part1_official_5() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_5).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 12240);
    }

    #[test]
    fn part1_official_6() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_6).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 13632);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 231);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 51);
    }

    #[test]
    fn part2_official_3() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_3).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 46);
    }

    #[test]
    fn part2_official_4() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_4).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 1445);
    }

    #[test]
    fn part2_official_5() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_5).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 669060);
    }

    #[test]
    fn part2_official_6() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_6).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 23340);
    }

    #[test]
    fn missing_operands() {
        assert!(shunting_yard("1 2 + 3", false).is_err());
        assert!(shunting_yard("1 + 2 3", true).is_err());
    }

    #[test]
    fn overflow_is_rejected() {
        let input = vec!["9"; 21].join("*");
        assert_eq!(shunting_yard(&input, false), Err(AocError::NoSolution));
    }

    #[test]
    fn unmatched_parentheses_are_rejected() {
        assert!(shunting_yard("1)", false).is_err());
        assert!(shunting_yard("(1", false).is_err());
        assert!(shunting_yard("1 + 2) * 3", false).is_err());
    }
}
