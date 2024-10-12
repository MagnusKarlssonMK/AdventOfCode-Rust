pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    directions: Vec<i32>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        fn translate(c: u8) -> i32 {
            match c {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            }
        }
        Self { directions: input.bytes().map(translate).collect() }
    }

    fn solve_part1(&self) -> i32 {
        self.directions.iter().sum()
    }

    fn solve_part2(&self) -> i32 {
        let mut floor = 0;
        let mut steps = 0;
        for v in self.directions.iter() {
            floor += v;
            steps += 1;
            if floor < 0 {
                break;
            }
        }
        steps
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("(())");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("()()");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("(((");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("(()(()(");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_5() {
        let testdata = String::from("))(((((");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_6() {
        let testdata = String::from("())");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), -1);
    }

    #[test]
    fn part1_example_7() {
        let testdata = String::from("))(");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), -1);
    }

    #[test]
    fn part1_example_8() {
        let testdata = String::from(")))");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), -3);
    }

    #[test]
    fn part1_example_9() {
        let testdata = String::from(")())())");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), -3);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(")");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("()())");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 5);
    }
}
