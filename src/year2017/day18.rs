//! # 2017 day 18 - Duet
use crate::aoc_util::error::*;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
enum Value {
    Literal(i64),
    Register(char),
}

impl FromStr for Value {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<i64>() {
            Ok(Self::Literal(v))
        } else if let Some(c) = s.chars().next()
            && s.len() == 1
        {
            Ok(Self::Register(c))
        } else {
            Err(AocError::Invalid(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Instruction {
    /// `snd X` plays a sound with a frequency equal to the value of X.
    Snd(Value),
    /// `set X Y` sets register X to the value of Y.
    Set(char, Value),
    /// `add X Y` increases register X by the value of Y.
    Add(char, Value),
    /// `mul X Y` sets register X to the result of multiplying the value contained in
    /// register X by the value of Y.
    Mul(char, Value),
    /// mod X Y sets register X to the remainder of dividing the value contained in
    /// register X by the value of Y (that is, it sets X to the result of X modulo Y).
    Mod(char, Value),
    /// rcv X recovers the frequency of the last sound played, but only when the value
    /// of X is not zero. (If it is zero, the command does nothing.)
    Rcv(Value),
    /// jgz X Y jumps with an offset of the value of Y, but only if the value of X is
    /// greater than zero. (An offset of 2 skips the next instruction, an offset of -1
    /// jumps to the previous instruction, and so on.)
    Jgz(Value, Value),
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
            ["snd", x] => Ok(Self::Snd(x.parse()?)),
            ["set", x, y] => Ok(Self::Set(x.chars().next().unwrap(), y.parse()?)),
            ["add", x, y] => Ok(Self::Add(x.chars().next().unwrap(), y.parse()?)),
            ["mul", x, y] => Ok(Self::Mul(x.chars().next().unwrap(), y.parse()?)),
            ["mod", x, y] => Ok(Self::Mod(x.chars().next().unwrap(), y.parse()?)),
            ["rcv", x] => Ok(Self::Rcv(x.parse()?)),
            ["jgz", x, y] => Ok(Self::Jgz(x.parse()?, y.parse()?)),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

#[derive(PartialEq)]
enum State {
    Idle,
    Receiving,
    Done,
}

struct Register {
    regs: HashMap<char, i64>,
}

impl Register {
    fn new() -> Self {
        Self {
            regs: HashMap::new(),
        }
    }

    fn get_value(&self, v: &Value) -> i64 {
        match v {
            Value::Literal(i) => *i,
            Value::Register(c) => *self.regs.get(c).unwrap_or(&0),
        }
    }

    fn set_value(&mut self, reg: char, v: &Value) {
        self.regs.insert(reg, self.get_value(v));
    }

    fn add_value(&mut self, reg: char, v: &Value) {
        let new_v = self.get_value(v);
        self.regs
            .entry(reg)
            .and_modify(|i| *i += new_v)
            .or_insert(new_v);
    }

    fn mul_value(&mut self, reg: char, v: &Value) {
        let new_v = self.get_value(v);
        self.regs
            .entry(reg)
            .and_modify(|i| *i *= new_v)
            .or_insert(0);
    }

    fn mod_value(&mut self, reg: char, v: &Value) {
        let new_v = self.get_value(v);
        self.regs
            .entry(reg)
            .and_modify(|i| *i %= new_v)
            .or_insert(0);
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
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<i64, AocError> {
        let mut sp = 0i64;
        let mut regs = Register::new();
        let mut sound = None;
        while (0..self.instructions.len() as i64).contains(&sp) {
            match &self.instructions[sp as usize] {
                Instruction::Snd(x) => sound = Some(regs.get_value(x)),
                Instruction::Set(x, y) => regs.set_value(*x, y),
                Instruction::Add(x, y) => regs.add_value(*x, y),
                Instruction::Mul(x, y) => regs.mul_value(*x, y),
                Instruction::Mod(x, y) => regs.mod_value(*x, y),
                Instruction::Rcv(x) => {
                    if regs.get_value(x) != 0
                        && let Some(s) = sound
                    {
                        return Ok(s);
                    }
                }
                Instruction::Jgz(x, y) => {
                    if regs.get_value(x) > 0 {
                        sp += regs.get_value(y);
                        continue;
                    }
                }
            }
            sp += 1;
        }
        Err(AocError::NoSolution)
    }

    fn solve_part2(&self) -> usize {
        let mut sp = [0i64, 0i64];
        let mut regs = [Register::new(), Register::new()];
        regs[0].regs.insert('p', 0);
        regs[1].regs.insert('p', 1);
        let mut states = [State::Idle, State::Idle];
        let mut inbox = [VecDeque::new(), VecDeque::new()];
        let mut running = 0usize;
        let mut send_count = 0;
        while states[running] == State::Idle
            || (states[running] == State::Receiving && !inbox[running].is_empty())
        {
            match &self.instructions[sp[running] as usize] {
                Instruction::Snd(x) => {
                    inbox[(running + 1) % 2].push_back(regs[running].get_value(x));
                    sp[running] += 1;
                    if running == 1 {
                        send_count += 1
                    }
                }
                Instruction::Set(x, y) => {
                    regs[running].set_value(*x, y);
                    sp[running] += 1;
                }
                Instruction::Add(x, y) => {
                    regs[running].add_value(*x, y);
                    sp[running] += 1;
                }
                Instruction::Mul(x, y) => {
                    regs[running].mul_value(*x, y);
                    sp[running] += 1;
                }
                Instruction::Mod(x, y) => {
                    regs[running].mod_value(*x, y);
                    sp[running] += 1;
                }
                Instruction::Rcv(x) => {
                    if let Some(m) = inbox[running].pop_front() {
                        if let Value::Register(r) = x {
                            regs[running].set_value(*r, &Value::Literal(m));
                        }
                        states[running] = State::Idle;
                        sp[running] += 1;
                    } else {
                        states[running] = State::Receiving;
                        running = (running + 1) % 2;
                    }
                }
                Instruction::Jgz(x, y) => {
                    if regs[running].get_value(x) > 0 {
                        sp[running] += regs[running].get_value(y);
                    } else {
                        sp[running] += 1;
                    }
                }
            }
            if !(0..self.instructions.len() as i64).contains(&sp[running]) {
                states[running] = State::Done;
                running = (running + 1) % 2;
            }
        }
        send_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    const TEST_OFFICIAL_2: &str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    // Exercises every part-2 opcode (set/add/mul/mod), both jgz branches and the DONE transition.
    const TEST_CUSTOM_1: &str = "set a 4
add a 3
mul a 2
mod a 5
jgz a 2
snd 9
snd a
set z 0
jgz z 2
snd 8
rcv b";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 4);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
