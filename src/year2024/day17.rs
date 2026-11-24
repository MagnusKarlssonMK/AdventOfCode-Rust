//! # 2024 day 17 - Chronospatial Computer
//!
//! ## Part 1
//!
//! Run the small VM (all instructions are opcode + operand) on the given registers and collect its
//! output.
//!
//! ## Part 2
//!
//! Rebuild the required initial value of register A three bits at a time, working backwards
//! through the program: extend each surviving candidate with every 3-bit value and keep those
//! whose first output byte matches the current program digit.
use crate::aoc_util::error::{AocError, OptionExt};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?,
        solution_data.solve_part2()?.to_string(),
    ))
}

enum Opcodes {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcodes {
    fn from_nbr(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Adv),
            1 => Some(Self::Bxl),
            2 => Some(Self::Bst),
            3 => Some(Self::Jnz),
            4 => Some(Self::Bxc),
            5 => Some(Self::Out),
            6 => Some(Self::Bdv),
            7 => Some(Self::Cdv),
            _ => None,
        }
    }
}

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    i_p: usize,
}

impl Computer {
    fn new(a: u64, b: u64, c: u64, program: &[u8]) -> Self {
        Self {
            a,
            b,
            c,
            program: program.to_vec(),
            i_p: 0,
        }
    }

    fn reboot(&mut self, new_a: u64) {
        self.a = new_a;
        self.b = 0;
        self.c = 0;
        self.i_p = 0;
    }

    fn get_combo_op(&self, n: u8) -> Result<u64, AocError> {
        match n {
            0..=3 => Ok(n as u64),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(AocError::Invalid(format!("invalid combo op: {n}"))),
        }
    }

    fn run(&mut self) -> Result<Vec<u8>, AocError> {
        let mut out = Vec::new();
        while self.i_p < self.program.len() {
            let op = Opcodes::from_nbr(self.program[self.i_p]).ctx("invalid opcode")?;
            let operand = *self.program.get(self.i_p + 1).ctx("missing operand")?;
            match op {
                Opcodes::Adv => {
                    let v = self.get_combo_op(operand)?;
                    self.a /= 2_u64.pow(v as u32);
                    self.i_p += 2;
                }
                Opcodes::Bxl => {
                    self.b ^= operand as u64;
                    self.i_p += 2;
                }
                Opcodes::Bst => {
                    self.b = self.get_combo_op(operand)? % 8;
                    self.i_p += 2;
                }
                Opcodes::Jnz => {
                    if self.a == 0 {
                        self.i_p += 2;
                    } else {
                        self.i_p = operand as usize;
                    }
                }
                Opcodes::Bxc => {
                    self.b ^= self.c;
                    self.i_p += 2;
                }
                Opcodes::Out => {
                    let v = self.get_combo_op(operand)?;
                    out.push((v % 8) as u8);
                    self.i_p += 2;
                }
                Opcodes::Bdv => {
                    let v = self.get_combo_op(operand)?;
                    self.b = self.a / 2_u64.pow(v as u32);
                    self.i_p += 2;
                }
                Opcodes::Cdv => {
                    let v = self.get_combo_op(operand)?;
                    self.c = self.a / 2_u64.pow(v as u32);
                    self.i_p += 2;
                }
            }
        }
        Ok(out)
    }
}

struct InputData {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
}

/// Consumes the next non-empty line and returns the value after `prefix`.
fn parse_line<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    prefix: &str,
) -> Result<&'a str, AocError> {
    lines
        .next()
        .ctx("missing line")?
        .strip_prefix(prefix)
        .ctx("unexpected field label")
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter(|line| !line.is_empty());
        Ok(Self {
            reg_a: parse_line(&mut lines, "Register A: ")?.parse()?,
            reg_b: parse_line(&mut lines, "Register B: ")?.parse()?,
            reg_c: parse_line(&mut lines, "Register C: ")?.parse()?,
            program: parse_line(&mut lines, "Program: ")?
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<String, AocError> {
        let mut computer = Computer::new(self.reg_a, self.reg_b, self.reg_c, &self.program);
        let out = computer.run()?;
        let result: String = out.iter().map(|n| n.to_string() + ",").collect();
        Ok(result.trim_end_matches(',').to_string())
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let mut solutions = vec![0];
        let mut computer = Computer::new(self.reg_a, self.reg_b, self.reg_c, &self.program);
        // Add digit by digit in the program in reverse order
        for p_digit in self.program.iter().rev() {
            let mut subprogram = Vec::new();
            for nbr in solutions {
                // For every possible solution from the previous iteration, shift it 3 bits
                // to the left and try values 0-7 in the 3 LSB
                for i in 0..8 {
                    let test_a = (nbr << 3) | i;
                    computer.reboot(test_a);
                    if computer.run()?.first() == Some(p_digit) {
                        subprogram.push(test_a);
                    }
                }
            }
            solutions = subprogram;
        }
        // Choose the smallest value found
        solutions.iter().min().copied().ok_or(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_OFFICIAL_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(
            solution_data.solve_part1().unwrap(),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 117440);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "",
            "Register A: 1\nRegister B: 2\nRegister C: 3",
            "Register A: 1\nRegister B: 2\nWrong: 3\n\nProgram: 0,1",
        ] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }

        let input = "Register A: x\nRegister B: 2\nRegister C: 3\n\nProgram: 0,1";
        assert!(matches!(
            input.parse::<InputData>(),
            Err(AocError::ParseInt(_))
        ));
    }

    #[test]
    fn program_without_operand_is_invalid() {
        let input = "Register A: 1\nRegister B: 0\nRegister C: 0\n\nProgram: 0";
        let solution_data: InputData = input.parse().unwrap();
        assert!(matches!(
            solution_data.solve_part1(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn program_without_output_has_no_solution() {
        let input = "Register A: 1\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1";
        let solution_data: InputData = input.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), "");
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }
}
