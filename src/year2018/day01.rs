use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    data: Vec<isize>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            data: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn solve_part1(&self) -> isize {
        self.data.iter().sum()
    }

    fn solve_part2(&self) -> isize {
        let mut i: usize = 0;
        let mut freq: isize = 0;
        let mut seen: HashSet<isize> = HashSet::new();
        seen.insert(freq);
        loop {
            freq += self.data[i];
            if seen.contains(&freq) {
                break;
            }
            seen.insert(freq);
            i = (i + 1) % self.data.len();
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("+1\n-2\n+3\n+1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("+1\n+1\n+1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("+1\n+1\n-2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("-1\n-2\n-3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), -6);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("+1\n-2\n+3\n+1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("+1\n-1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from("+3\n+3\n+4\n-2\n-4");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 10);
    }

    #[test]
    fn part2_example_4() {
        let testdata = String::from("-6\n+3\n+8\n+5\n-6");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 5);
    }

    #[test]
    fn part2_example_5() {
        let testdata = String::from("+7\n+7\n-2\n-7\n-4");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 14);
    }
}
