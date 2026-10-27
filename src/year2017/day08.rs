//! # 2017 day 8 - I Heard You Like Registers
use std::{collections::HashMap, error::Error, str::FromStr};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug, Clone, Copy)]
enum CondOp {
    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=
    Eq, // ==
    Ne, // !=
}

impl FromStr for CondOp {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(CondOp::Gt),
            "<" => Ok(CondOp::Lt),
            ">=" => Ok(CondOp::Ge),
            "<=" => Ok(CondOp::Le),
            "==" => Ok(CondOp::Eq),
            "!=" => Ok(CondOp::Ne),
            _ => Err(AocError::Invalid(format!(
                "unknown condition operator: {s}"
            ))),
        }
    }
}

impl CondOp {
    fn eval(self, left: i32, right: i32) -> bool {
        match self {
            CondOp::Gt => left > right,
            CondOp::Lt => left < right,
            CondOp::Ge => left >= right,
            CondOp::Le => left <= right,
            CondOp::Eq => left == right,
            CondOp::Ne => left != right,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    target: String,
    delta: i32,
    cond_reg: String,
    cond_op: CondOp,
    cond_val: i32,
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        match parts.as_slice() {
            [target, op_str, amount, "if", cond_reg, cond_op, cond_val] => {
                let amount: i32 = amount.parse()?;
                let delta = match *op_str {
                    "inc" => amount,
                    "dec" => -amount,
                    other => return Err(AocError::Invalid(format!("unknown op: {other}"))),
                };

                Ok(Instruction {
                    target: target.to_string(),
                    delta,
                    cond_reg: cond_reg.to_string(),
                    cond_op: cond_op.parse()?,
                    cond_val: cond_val.parse()?,
                })
            }
            _ => Err(AocError::Invalid(s.to_string())),
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
                .lines()
                .map(Instruction::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> (i32, i32) {
        let mut registers = HashMap::new();
        let mut max_val = 0;
        for instr in &self.instructions {
            let cond_left = *registers.get(&instr.cond_reg).unwrap_or(&0);
            if !instr.cond_op.eval(cond_left, instr.cond_val) {
                continue;
            }

            let entry = registers.entry(instr.target.clone()).or_insert(0);
            *entry += instr.delta;
            if *entry > max_val {
                max_val = *entry;
            }
        }
        (*registers.values().max().unwrap_or(&0), max_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    /// The condition is never true, so no register is ever modified.
    const TEST_CUSTOM_1: &str = "a inc 1 if b > 0";

    /// Exercises the <= and != condition operators, which the official example does not use.
    const TEST_CUSTOM_2: &str = "b inc 1 if a <= 0
b inc 1 if a != 1";

    #[test]
    fn parts_1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1);
        assert_eq!(p2, 10);
    }

    #[test]
    fn parts_1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
    }

    #[test]
    fn parts_1_2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 2);
        assert_eq!(p2, 2);
    }

    #[test]
    fn parser_invalid_input() {
        let err = InputData::from_str("b inc 3 if f <> 3").unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)));
        let err = InputData::from_str("b dnc 3 if f > 3").unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)));
        let err = InputData::from_str("b dec 3 if f <").unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)));
        let err = InputData::from_str("b inc f if f == 3").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
