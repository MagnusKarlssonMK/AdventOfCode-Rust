//! # 2024 day 17 - Chronospatial Computer
pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
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
    a: usize,
    b: usize,
    c: usize,
    program: Vec<u8>,
    i_p: usize,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: &[u8]) -> Self {
        Self {
            a,
            b,
            c,
            program: program.to_vec(),
            i_p: 0,
        }
    }

    fn reboot(&mut self, new_a: usize) {
        self.a = new_a;
        self.b = 0;
        self.c = 0;
        self.i_p = 0;
    }

    fn get_combo_op(&self, n: u8) -> Option<usize> {
        match n {
            0..=3 => Some(n as usize),
            4 => Some(self.a),
            5 => Some(self.b),
            6 => Some(self.c),
            _ => None,
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        while (0..self.program.len()).contains(&self.i_p) {
            if let Some(op) = Opcodes::from_nbr(self.program[self.i_p]) {
                match op {
                    Opcodes::Adv => {
                        let v = self.get_combo_op(self.program[self.i_p + 1]).unwrap();
                        self.a /= 2_usize.pow(v as u32);
                        self.i_p += 2;
                    }
                    Opcodes::Bxl => {
                        let v = self.program[self.i_p + 1] as usize;
                        self.b ^= v;
                        self.i_p += 2;
                    }
                    Opcodes::Bst => {
                        self.b = self.get_combo_op(self.program[self.i_p + 1]).unwrap() % 8;
                        self.i_p += 2;
                    }
                    Opcodes::Jnz => {
                        if self.a == 0 {
                            self.i_p += 2;
                        } else {
                            self.i_p = self.program[self.i_p + 1] as usize;
                        }
                    }
                    Opcodes::Bxc => {
                        self.b ^= self.c;
                        self.i_p += 2;
                    }
                    Opcodes::Out => {
                        let v = self.get_combo_op(self.program[self.i_p + 1]).unwrap();
                        out.push((v % 8) as u8);
                        self.i_p += 2;
                    }
                    Opcodes::Bdv => {
                        let v = self.get_combo_op(self.program[self.i_p + 1]).unwrap();
                        self.b = self.a / 2_usize.pow(v as u32);
                        self.i_p += 2;
                    }
                    Opcodes::Cdv => {
                        let v = self.get_combo_op(self.program[self.i_p + 1]).unwrap();
                        self.c = self.a / 2_usize.pow(v as u32);
                        self.i_p += 2;
                    }
                }
            }
        }
        out
    }
}

struct InputData {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<u8>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let (regs, prog) = input.split_once("\n\n").unwrap();
        let mut regs = regs.lines();
        let reg_a = regs
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap();
        let reg_b = regs
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap();
        let reg_c = regs
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap();
        Self {
            reg_a,
            reg_b,
            reg_c,
            program: prog
                .strip_prefix("Program: ")
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect(),
        }
    }

    fn solve_part1(&self) -> String {
        let mut computer = Computer::new(self.reg_a, self.reg_b, self.reg_c, &self.program);
        let out = computer.run();
        let result: String = out.iter().map(|n| n.to_string() + ",").collect();
        result.strip_suffix(',').unwrap().to_string()
    }

    fn solve_part2(&self) -> usize {
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
                    if computer.run()[0] == *p_digit {
                        subprogram.push(test_a);
                    }
                }
            }
            solutions = subprogram;
        }
        // Choose the smallest value found
        *solutions.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(
            solution_data.solve_part1(),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 117440);
    }
}
