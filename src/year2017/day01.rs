pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    sequence: Vec<usize>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            sequence: input
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect(),
        }
    }

    fn get_captcha(&self, halfway: bool) -> usize {
        let v_offset = if !halfway { 1 } else { self.sequence.len() / 2 };
        let mut total: usize = 0;
        for (i, nbr) in self.sequence.iter().enumerate() {
            if *nbr == self.sequence[(i + v_offset) % self.sequence.len()] {
                total += *nbr;
            }
        }
        total
    }

    fn solve_part1(&self) -> usize {
        self.get_captcha(false)
    }

    fn solve_part2(&self) -> usize {
        self.get_captcha(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("1122");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("1111");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("1234");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("91212129");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 9);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("1212");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 6);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("1221");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from("123425");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 4);
    }

    #[test]
    fn part2_example_4() {
        let testdata = String::from("123123");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 12);
    }

    #[test]
    fn part2_example_5() {
        let testdata = String::from("12131415");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
