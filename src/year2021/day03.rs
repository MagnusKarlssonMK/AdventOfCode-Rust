pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    numbers: Vec<String>,
    width: usize
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let w = input.lines().nth(0).unwrap().len();
        Self {
            numbers: input.lines()
                .map(|l| l.to_string())
                .collect(),
            width: w
        }
    }

    fn solve_part1(&self) -> usize {
        println!("{:?}", usize::from_str_radix(&self.numbers[0], 2).unwrap());
        println!("{}", self.width);
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
        let testdata = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 198);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 230);
    }
}
