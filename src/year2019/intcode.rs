//! # Intcode
//!
//! Contains the implementation of the Intcode emulator.
use std::collections::VecDeque;

const ADD_MEMORY_SIZE: usize = 2_000;

pub enum State {
    Input,
    Output(isize),
    Halted,
}

pub struct Intcode {
    i_p: usize,
    program: Vec<isize>,
    input_buffer: VecDeque<isize>,
}

enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_int(n: isize) -> Self {
        match n % 10 {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => unreachable!(),
        }
    }
}

impl Intcode {
    pub fn new(prog: &[isize]) -> Self {
        let mut program = Vec::with_capacity(prog.len() + ADD_MEMORY_SIZE);
        program.extend_from_slice(prog);
        program.resize(prog.len() + ADD_MEMORY_SIZE, 0);
        Self {
            i_p: 0,
            program,
            input_buffer: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> State {
        loop {
            let op_code = self.program[self.i_p];
            match op_code % 100 {
                1 => {
                    // Add
                    let a_vec: Vec<usize> = (1..=3)
                        .map(|i| {
                            self.get_address(
                                ParameterMode::from_int(op_code / (10_isize.pow(i as u32 + 1))),
                                i,
                            )
                        })
                        .collect();
                    self.program[a_vec[2]] = self.program[a_vec[0]] + self.program[a_vec[1]];
                    self.i_p += 4;
                }
                2 => {
                    // Mul
                    let a_vec: Vec<usize> = (1..=3)
                        .map(|i| {
                            self.get_address(
                                ParameterMode::from_int(op_code / (10_isize.pow(i as u32 + 1))),
                                i,
                            )
                        })
                        .collect();
                    self.program[a_vec[2]] = self.program[a_vec[0]] * self.program[a_vec[1]];
                    self.i_p += 4;
                }
                3 => {
                    // Input
                    if let Some(v) = self.input_buffer.pop_front() {
                        let address = self.get_address(ParameterMode::from_int(op_code / 100), 1);
                        self.program[address] = v;
                        self.i_p += 2;
                    } else {
                        break State::Input;
                    }
                }
                4 => {
                    // Output
                    let address = self.get_address(ParameterMode::from_int(op_code / 100), 1);
                    let out = self.program[address];
                    self.i_p += 2;
                    break State::Output(out);
                }
                5 => {
                    // Jump-if-true
                    let first_address = self.get_address(ParameterMode::from_int(op_code / 100), 1);
                    if self.program[first_address] != 0 {
                        let second_address =
                            self.get_address(ParameterMode::from_int(op_code / 1000), 2);
                        self.i_p = self.program[second_address].try_into().unwrap();
                    } else {
                        self.i_p += 3;
                    }
                }
                6 => {
                    // Jump-if-false
                    let first_address = self.get_address(ParameterMode::from_int(op_code / 100), 1);
                    if self.program[first_address] == 0 {
                        let second_address =
                            self.get_address(ParameterMode::from_int(op_code / 1000), 2);
                        self.i_p = self.program[second_address].try_into().unwrap();
                    } else {
                        self.i_p += 3;
                    }
                }
                7 => {
                    // Less-than
                    let a_vec: Vec<usize> = (1..=3)
                        .map(|i| {
                            self.get_address(
                                ParameterMode::from_int(op_code / (10_isize.pow(i as u32 + 1))),
                                i,
                            )
                        })
                        .collect();
                    self.program[a_vec[2]] = if self.program[a_vec[0]] < self.program[a_vec[1]] {
                        1
                    } else {
                        0
                    };
                    self.i_p += 4;
                }
                8 => {
                    // Equals
                    let a_vec: Vec<usize> = (1..=3)
                        .map(|i| {
                            self.get_address(
                                ParameterMode::from_int(op_code / (10_isize.pow(i as u32 + 1))),
                                i,
                            )
                        })
                        .collect();
                    self.program[a_vec[2]] = if self.program[a_vec[0]] == self.program[a_vec[1]] {
                        1
                    } else {
                        0
                    };
                    self.i_p += 4;
                }
                _ => {
                    // Halt
                    break State::Halted;
                }
            }
        }
    }

    pub fn input_value(&mut self, val: isize) {
        self.input_buffer.push_back(val);
    }

    pub fn overwrite_pos(&mut self, pos: usize, val: isize) {
        self.program[pos] = val;
    }

    pub fn read_pos(&self, pos: usize) -> isize {
        self.program[pos]
    }

    #[inline]
    fn get_address(&self, mode: ParameterMode, offset: usize) -> usize {
        match mode {
            ParameterMode::Position => self.program[self.i_p + offset] as usize,
            ParameterMode::Immediate => self.i_p + offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_example_1() {
        let mut testcpu = Intcode::new(&[1, 0, 0, 0, 99]);
        testcpu.run();
        for (i, v) in [2, 0, 0, 0, 99].iter().enumerate() {
            assert_eq!(*v, testcpu.program[i]);
        }
    }

    #[test]
    fn day2_part1_example_2() {
        let mut testcpu = Intcode::new(&[2, 3, 0, 3, 99]);
        testcpu.run();
        for (i, v) in [2, 3, 0, 6, 99].iter().enumerate() {
            assert_eq!(*v, testcpu.program[i]);
        }
    }

    #[test]
    fn day2_part1_example_3() {
        let mut testcpu = Intcode::new(&[2, 4, 4, 5, 99, 0]);
        testcpu.run();
        for (i, v) in [2, 4, 4, 5, 99, 9801].iter().enumerate() {
            assert_eq!(*v, testcpu.program[i]);
        }
    }

    #[test]
    fn day2_part1_example_4() {
        let mut testcpu = Intcode::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        testcpu.run();
        for (i, v) in [30, 1, 1, 4, 2, 5, 6, 0, 99].iter().enumerate() {
            assert_eq!(*v, testcpu.program[i]);
        }
    }
}
