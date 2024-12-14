const ADD_MEMORY_SIZE: usize = 2_000;

pub enum State {
    Halted,
}

pub struct Intcode {
    i_p: usize,
    program: Vec<usize>,
}

impl Intcode {
    pub fn new(prog: &[usize]) -> Self {
        let mut program = Vec::with_capacity(prog.len() + ADD_MEMORY_SIZE);
        program.extend_from_slice(prog);
        program.resize(prog.len() + ADD_MEMORY_SIZE, 0);
        Self { i_p: 0, program }
    }

    pub fn run(&mut self) -> State {
        loop {
            let op_code = self.program[self.i_p];
            match op_code {
                1 => {
                    // Add
                    let pars: Vec<usize> = (1..=3).map(|i| self.program[self.i_p + i]).collect();
                    self.program[pars[2]] = self.program[pars[0]] + self.program[pars[1]];
                    self.i_p += 4;
                }
                2 => {
                    // Mul
                    let pars: Vec<usize> = (1..=3).map(|i| self.program[self.i_p + i]).collect();
                    self.program[pars[2]] = self.program[pars[0]] * self.program[pars[1]];
                    self.i_p += 4;
                }
                _ => {
                    // Halt
                    break State::Halted;
                }
            }
        }
    }

    pub fn overwrite_pos(&mut self, pos: usize, val: usize) {
        self.program[pos] = val;
    }

    pub fn read_pos(&self, pos: usize) -> usize {
        self.program[pos]
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
