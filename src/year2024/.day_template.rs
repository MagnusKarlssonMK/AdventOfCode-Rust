pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    data: Vec<String>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            data: input.lines().map(|line| line.to_string()).collect()
        }
    }

    fn solve_part1(&self) -> usize {
        for line in &self.data {
            println!("{}", line);
        }
        1
    }

    fn solve_part2(&self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
