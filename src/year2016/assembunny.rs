//! Emulator for the Assembunny computer used in days 12 and 23
use crate::aoc_util::error::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Integer(i32),
    Reg(char),
}

impl Value {
    fn new(s: &str) -> Result<Value, AocError> {
        if let Ok(i) = s.parse::<i32>() {
            Ok(Value::Integer(i))
        } else {
            Ok(Value::Reg(str_to_reg(s)?))
        }
    }
}

const REGISTER_CHARS: [char; 4] = ['a', 'b', 'c', 'd'];

fn str_to_reg(s: &str) -> Result<char, AocError> {
    if s.chars().count() != 1 {
        Err(AocError::Invalid(format!("reg: {s}").to_string()))
    } else {
        let c = s.chars().next().unwrap();
        if REGISTER_CHARS.contains(&c) {
            Ok(c)
        } else {
            Err(AocError::Invalid(format!("reg 1: {s}").to_string()))
        }
    }
}

pub struct Registers {
    regs: HashMap<char, i32>,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            regs: REGISTER_CHARS.iter().map(|c| (*c, 0)).collect(),
        }
    }
}

impl Registers {
    pub fn get_reg(&self, reg: char) -> Result<i32, AocError> {
        self.regs.get(&reg).ctx("undefined register").copied()
    }

    pub fn set_reg(&mut self, reg: char, val: i32) {
        if let Some(v) = self.regs.get_mut(&reg) {
            *v = val;
        }
    }

    fn inc_reg(&mut self, reg: char) {
        if let Some(v) = self.regs.get_mut(&reg) {
            *v += 1
        }
    }

    fn dec_reg(&mut self, reg: char) {
        if let Some(v) = self.regs.get_mut(&reg) {
            *v -= 1
        }
    }

    fn get_val(&self, v: &Value) -> Result<i32, AocError> {
        match *v {
            Value::Integer(i) => Ok(i),
            Value::Reg(r) => self.get_reg(r),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// (x, y) - Copy value of x to reg y
    Cpy(Value, char),
    /// (x) - Increase reg x by 1
    Inc(char),
    /// (x) - Decrease reg x by 1
    Dec(char),
    /// (x, y) - If x is NOT zero, jump an instruction with value of y
    Jnz(Value, Value),
    /// (x) - Toggle register x (Only used in day 23)
    Tgl(char),
    /// (x, y, z) - Reg z incremented by the product of values of x and y (Custom made opcode not in input, only inserted by optimizer)
    Mul(Value, Value, char),
    /// (x, y) - Increase reg y by x (Custom made opcode not in input, only inserted by optimizer)
    Add(Value, char),
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ctx_invalid(format!("instruction {s}").to_string())?;
        match left {
            "cpy" => {
                let (v1, v2) = right
                    .split_once(' ')
                    .ctx_invalid(format!("instruction {s}").to_string())?;
                Ok(Instruction::Cpy(Value::new(v1)?, str_to_reg(v2)?))
            }
            "inc" => Ok(Instruction::Inc(str_to_reg(right)?)),
            "dec" => Ok(Instruction::Dec(str_to_reg(right)?)),
            "jnz" => {
                let (v1, v2) = right
                    .split_once(' ')
                    .ctx_invalid(format!("instruction {s}").to_string())?;
                Ok(Instruction::Jnz(Value::new(v1)?, Value::new(v2)?))
            }
            "tgl" => Ok(Instruction::Tgl(str_to_reg(right)?)),
            _ => Err(AocError::Invalid(format!("instruction {s}").to_string())),
        }
    }
}

impl Instruction {
    fn get_toggled_op(&self) -> Option<Instruction> {
        match self {
            Instruction::Inc(x) => Some(Instruction::Dec(*x)),
            Instruction::Dec(x) | Instruction::Tgl(x) => Some(Instruction::Inc(*x)),
            Instruction::Cpy(x, y) => Some(Instruction::Jnz(*x, Value::Reg(*y))),
            Instruction::Jnz(x, y) => match y {
                Value::Integer(_) => None, // Invalid mapping; instruction should be ignored
                Value::Reg(v) => Some(Instruction::Cpy(*x, *v)),
            },
            _ => Some(*self),
        }
    }
}

/// Optimizes the program by replacing common single-step addition and multiplication loops with
/// the custom `Add`/`Mul` opcodes. The instructions the loop consisted of are overwritten: the
/// counter registers are reset to 0 and the leftover slots are filled with no-ops.
pub fn optimize_program(program: &mut [Instruction]) {
    // First check for the Mul pattern, since its first three instructions also match the Add
    // pattern below:
    //
    //     inc x
    //     dec y
    //     jnz y -2    (loop while y != 0)
    //     dec z
    //     jnz z -5    (loop while z != 0)
    //
    // which is equivalent to `x += y * z`, with y and z ending up as 0.
    for i in 0..program.len().saturating_sub(4) {
        if let (
            Instruction::Inc(x),
            Instruction::Dec(y),
            Instruction::Jnz(Value::Reg(y2), Value::Integer(-2)),
            Instruction::Dec(z),
            Instruction::Jnz(Value::Reg(z2), Value::Integer(-5)),
        ) = (
            program[i],
            program[i + 1],
            program[i + 2],
            program[i + 3],
            program[i + 4],
        ) && y == y2
            && z == z2
        {
            program[i] = Instruction::Mul(Value::Reg(y), Value::Reg(z), x);
            program[i + 1] = Instruction::Cpy(Value::Integer(0), y);
            program[i + 2] = Instruction::Cpy(Value::Integer(0), z);
            program[i + 3] = Instruction::Jnz(Value::Integer(0), Value::Integer(0));
            program[i + 4] = Instruction::Jnz(Value::Integer(0), Value::Integer(0));
        }
    }

    // Then check for the two Add patterns:
    //
    //     inc x               dec x
    //     dec y (or inc y)    inc y
    //     jnz y -2            jnz x -2
    //
    // which are equivalent to `x += y` and `y += x` respectively, with the counter ending up as 0.
    for i in 0..program.len().saturating_sub(2) {
        match (program[i], program[i + 1], program[i + 2]) {
            (
                Instruction::Inc(x),
                Instruction::Dec(y) | Instruction::Inc(y),
                Instruction::Jnz(Value::Reg(y2), Value::Integer(-2)),
            ) if y == y2 => {
                program[i] = Instruction::Add(Value::Reg(y), x);
                program[i + 1] = Instruction::Cpy(Value::Integer(0), y);
                program[i + 2] = Instruction::Jnz(Value::Integer(0), Value::Integer(0));
            }
            (
                Instruction::Dec(x),
                Instruction::Inc(y),
                Instruction::Jnz(Value::Reg(x2), Value::Integer(-2)),
            ) if x == x2 => {
                program[i] = Instruction::Add(Value::Reg(x), y);
                program[i + 1] = Instruction::Cpy(Value::Integer(0), x);
                program[i + 2] = Instruction::Jnz(Value::Integer(0), Value::Integer(0));
            }
            _ => {}
        }
    }
}

pub fn run_program(program: &[Instruction], reg: &mut Registers) -> Result<(), AocError> {
    let mut sp: i32 = 0;
    let mut toggle_list = vec![false; program.len()];
    while (0..program.len() as i32).contains(&sp) {
        let mut instr = program[sp as usize];
        if toggle_list[sp as usize]
            && let Some(i) = instr.get_toggled_op()
        {
            instr = i;
        }
        match instr {
            Instruction::Cpy(x, y) => {
                reg.set_reg(y, reg.get_val(&x)?);
            }
            Instruction::Inc(x) => {
                reg.inc_reg(x);
            }
            Instruction::Dec(x) => {
                reg.dec_reg(x);
            }
            Instruction::Jnz(x, y) => {
                if reg.get_val(&x)? != 0 {
                    sp += reg.get_val(&y)?;
                    continue; // To not increment sp once more after match
                }
            }
            Instruction::Tgl(x) => {
                let val = reg.get_reg(x)? + sp;
                if (0..toggle_list.len() as i32).contains(&val) {
                    toggle_list[val as usize] = !toggle_list[val as usize];
                }
            }
            Instruction::Mul(x, y, z) => {
                let val = reg.get_reg(z)? + (reg.get_val(&x)? * reg.get_val(&y)?);
                reg.set_reg(z, val);
            }
            Instruction::Add(x, y) => {
                let val = reg.get_reg(y)? + reg.get_val(&x)?;
                reg.set_reg(y, val);
            }
        }
        sp += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn parse(s: &str) -> Vec<Instruction> {
        s.lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    #[test]
    fn optimizer_replaces_multiplication_loop() {
        let mut program = parse("inc a\ndec b\njnz b -2\ndec c\njnz c -5");
        optimize_program(&mut program);
        assert_eq!(
            program,
            vec![
                Instruction::Mul(Value::Reg('b'), Value::Reg('c'), 'a'),
                Instruction::Cpy(Value::Integer(0), 'b'),
                Instruction::Cpy(Value::Integer(0), 'c'),
                Instruction::Jnz(Value::Integer(0), Value::Integer(0)),
                Instruction::Jnz(Value::Integer(0), Value::Integer(0)),
            ]
        );
    }

    #[test]
    fn optimizer_replaces_increment_decrement_add_loop() {
        let mut program = parse("inc a\ndec b\njnz b -2");
        optimize_program(&mut program);
        assert_eq!(
            program,
            vec![
                Instruction::Add(Value::Reg('b'), 'a'),
                Instruction::Cpy(Value::Integer(0), 'b'),
                Instruction::Jnz(Value::Integer(0), Value::Integer(0)),
            ]
        );
    }

    #[test]
    fn optimizer_replaces_decrement_increment_add_loop() {
        let mut program = parse("dec a\ninc b\njnz a -2");
        optimize_program(&mut program);
        assert_eq!(
            program,
            vec![
                Instruction::Add(Value::Reg('a'), 'b'),
                Instruction::Cpy(Value::Integer(0), 'a'),
                Instruction::Jnz(Value::Integer(0), Value::Integer(0)),
            ]
        );
    }

    #[test]
    fn mul_and_add_opcodes_update_destination_register() {
        let mut reg = Registers::default();
        reg.set_reg('b', 3);
        reg.set_reg('c', 4);
        run_program(
            &[
                Instruction::Mul(Value::Reg('b'), Value::Reg('c'), 'a'),
                Instruction::Add(Value::Reg('c'), 'a'),
            ],
            &mut reg,
        )
        .unwrap();
        // a = b * c = 12, then a += c = 16
        assert_eq!(reg.get_reg('a').unwrap(), 16);
    }

    #[test]
    fn optimized_multiplication_program_computes_product() {
        let mut program = parse("cpy 3 b\ncpy 4 c\ninc a\ndec b\njnz b -2\ndec c\njnz c -5");
        optimize_program(&mut program);
        let mut reg = Registers::default();
        run_program(&program, &mut reg).unwrap();
        assert_eq!(reg.get_reg('a').unwrap(), 12);
        assert_eq!(reg.get_reg('b').unwrap(), 0);
        assert_eq!(reg.get_reg('c').unwrap(), 0);
    }
}
