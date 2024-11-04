pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    puzzle_input: usize
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { puzzle_input: input.parse().unwrap() }
    }

    fn solve_part1(&self) -> usize {
        println!("{}", self.puzzle_input);
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
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("12");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("23");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("1024");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 31);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("750");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 806);
    }
}
