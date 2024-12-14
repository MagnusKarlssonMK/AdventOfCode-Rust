pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    measurements: Vec<usize>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            measurements: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn get_depth(&self, windowsize: usize) -> usize {
        self.measurements
            .windows(windowsize + 1)
            .filter(|w| w[0] < w[windowsize])
            .count()
    }

    fn solve_part1(&self) -> usize {
        self.get_depth(1)
    }

    fn solve_part2(&self) -> usize {
        self.get_depth(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 5);
    }
}
