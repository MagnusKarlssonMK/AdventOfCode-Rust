pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    instructions: Vec<i32>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            instructions: input.lines().map(|i| i.parse().unwrap()).collect(),
        }
    }

    fn run_program(&self, strange: bool) -> usize {
        let mut program = self.instructions.to_vec();
        let mut sp: i32 = 0;
        let mut count: usize = 0;
        let program_length = i32::try_from(program.len()).unwrap();
        while 0 <= sp && sp < program_length {
            count += 1;
            let idx = sp as usize;
            let val = program[idx];
            if strange && val >= 3 {
                program[idx] -= 1;
            } else {
                program[idx] += 1;
            }
            sp += val;
        }
        count
    }

    fn solve_part1(&self) -> usize {
        self.run_program(false)
    }

    fn solve_part2(&self) -> usize {
        self.run_program(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("0\n3\n0\n1\n-3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("0\n3\n0\n1\n-3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 10);
    }
}
