//! # 2024 day 7 - Bridge Repair
//!
//! Solves both parts in one go through a recursive function, which attempts to
//! fully validate each equation, and only resorts to use the concatenation
//! operation if it's the only way to succeed at the validation. The state of
//! whether or not concatenation has been used combined with the calculated value
//! is carried up through the recursion chain through the return value.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(PartialEq)]
enum CalibrationResult {
    Ok(usize),
    ConcatinatedOk(usize),
    NotOk,
}

struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        Ok(Self {
            test_value: left.parse().unwrap(),
            numbers: right
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        })
    }
}

impl Equation {
    fn calibrate(&self) -> CalibrationResult {
        self.validate(self.numbers[0], &self.numbers[1..])
    }

    fn validate(&self, total: usize, nbrs: &[usize]) -> CalibrationResult {
        if nbrs.is_empty() {
            if total == self.test_value {
                CalibrationResult::Ok(total)
            } else {
                CalibrationResult::NotOk
            }
        } else if total > self.test_value {
            CalibrationResult::NotOk
        } else {
            // Prioritize Ok result, only start concatenating if necessary
            let add_result = self.validate(total + nbrs[0], &nbrs[1..]);
            if let CalibrationResult::Ok(_) = add_result {
                return add_result;
            }
            let mul_result = self.validate(total * nbrs[0], &nbrs[1..]);
            if let CalibrationResult::Ok(_) = mul_result {
                return mul_result;
            }
            if let CalibrationResult::ConcatinatedOk(_) = add_result {
                return add_result;
            } else if let CalibrationResult::ConcatinatedOk(_) = mul_result {
                return mul_result;
            } else {
                let conc_result = self.validate(total * concatinate(nbrs[0]) + nbrs[0], &nbrs[1..]);
                match conc_result {
                    CalibrationResult::Ok(v) => return CalibrationResult::ConcatinatedOk(v),
                    CalibrationResult::ConcatinatedOk(v) => {
                        return CalibrationResult::ConcatinatedOk(v)
                    }
                    CalibrationResult::NotOk => (),
                }
            }
            CalibrationResult::NotOk
        }
    }
}

fn concatinate(right: usize) -> usize {
    let mut multiplier = 10;
    while multiplier <= right {
        multiplier *= 10;
    }
    multiplier
}

struct InputData {
    equations: Vec<Equation>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            equations: s
                .lines()
                .map(|line| Equation::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        for eq in &self.equations {
            match eq.calibrate() {
                CalibrationResult::Ok(v) => {
                    p1 += v;
                    p2 += v;
                }
                CalibrationResult::ConcatinatedOk(v) => p2 += v,
                CalibrationResult::NotOk => (),
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3749);
        assert_eq!(p2, 11387);
    }
}
