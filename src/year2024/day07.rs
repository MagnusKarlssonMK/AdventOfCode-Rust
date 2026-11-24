//! # 2024 day 7 - Bridge Repair
//!
//! Solves both parts in one go through a recursive function, which attempts to
//! fully validate each equation, and only resorts to use the concatenation
//! operation if it's the only way to succeed at the validation. The state of
//! whether or not concatenation has been used combined with the calculated value
//! is carried up through the recursion chain through the return value.
use crate::aoc_util::error::AocError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(PartialEq)]
enum CalibrationResult {
    Ok(u64),
    ConcatenatedOk(u64),
    NotOk,
}

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(": ")
            .ok_or_else(|| AocError::Invalid(format!("invalid equation: {s}")))?;
        let numbers: Vec<_> = right
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        if numbers.is_empty() {
            return Err(AocError::Invalid(format!("invalid equation: {s}")));
        }
        Ok(Self {
            test_value: left.parse()?,
            numbers,
        })
    }
}

impl Equation {
    fn calibrate(&self) -> CalibrationResult {
        self.validate(self.numbers[0], &self.numbers[1..])
    }

    fn validate(&self, total: u64, nbrs: &[u64]) -> CalibrationResult {
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
            if let CalibrationResult::ConcatenatedOk(_) = add_result {
                return add_result;
            } else if let CalibrationResult::ConcatenatedOk(_) = mul_result {
                return mul_result;
            } else {
                let conc_result = self.validate(total * concatenate(nbrs[0]) + nbrs[0], &nbrs[1..]);
                match conc_result {
                    CalibrationResult::Ok(v) => return CalibrationResult::ConcatenatedOk(v),
                    CalibrationResult::ConcatenatedOk(v) => {
                        return CalibrationResult::ConcatenatedOk(v);
                    }
                    CalibrationResult::NotOk => (),
                }
            }
            CalibrationResult::NotOk
        }
    }
}

fn concatenate(right: u64) -> u64 {
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
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            equations: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> (u64, u64) {
        let mut p1 = 0;
        let mut p2 = 0;
        for eq in &self.equations {
            match eq.calibrate() {
                CalibrationResult::Ok(v) => {
                    p1 += v;
                    p2 += v;
                }
                CalibrationResult::ConcatenatedOk(v) => p2 += v,
                CalibrationResult::NotOk => (),
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3749);
        assert_eq!(p2, 11387);
    }

    #[test]
    fn parts1_2_custom_1() {
        // Input with single digit equations
        let solution_data: InputData = "10: 9\n4: 4".parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 4);
        assert_eq!(p2, 4);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["10: ", "10:4", "10; 4"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
        for input in ["1f: 10 20", "10: 1f 20"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::ParseInt(_))
            ));
        }
    }
}
