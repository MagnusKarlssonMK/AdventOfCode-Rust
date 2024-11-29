pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve_parts_1_2();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData<'a> {
    stream: &'a str
}

impl <'a>InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self {
            stream: input
        }
    }

    fn solve_parts_1_2(&self) -> (usize, usize) {
        let mut tokens = self.stream.chars();
        let mut garbage_count = 0;
        let mut score = 0;
        let mut level = 1;
        while let Some(token) = tokens.next() {
            match token {
                '<' => {
                    while let Some(garbage) = tokens.next() {
                        match garbage {
                            '!' => { tokens.next(); }
                            '>' => break,
                            _ => garbage_count += 1,
                        }
                    }
                }
                '{' => {
                    score += level;
                    level += 1;
                }
                '}' => {
                    level -= 1;
                }
                _ => ()
            }
        }
        (score, garbage_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let testdata = String::from("{}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 1);
    }

    #[test]
    fn part_1_example_2() {
        let testdata = String::from("{{{}}}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 6);
    }

    #[test]
    fn part_1_example_3() {
        let testdata = String::from("{{},{}}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 5);
    }

    #[test]
    fn part_1_example_4() {
        let testdata = String::from("{{{},{},{{}}}}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 16);
    }

    #[test]
    fn part_1_example_5() {
        let testdata = String::from("{<a>,<a>,<a>,<a>}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 1);
    }

    #[test]
    fn part_1_example_6() {
        let testdata = String::from("{{<ab>},{<ab>},{<ab>},{<ab>}}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 9);
    }

    #[test]
    fn part_1_example_7() {
        let testdata = String::from("{{<!!>},{<!!>},{<!!>},{<!!>}}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 9);
    }

    #[test]
    fn part_1_example_8() {
        let testdata = String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 3);
    }

    #[test]
    fn part_2_example_1() {
        let testdata = String::from("<>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_example_2() {
        let testdata = String::from("<random characters>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 17);
    }

    #[test]
    fn part_2_example_3() {
        let testdata = String::from("<<<<>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 3);
    }

    #[test]
    fn part_2_example_4() {
        let testdata = String::from("<{!>}>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 2);
    }

    #[test]
    fn part_2_example_5() {
        let testdata = String::from("<!!>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_example_6() {
        let testdata = String::from("<!!!>>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_example_7() {
        let testdata = String::from("<{o\"i!a,<{i<a>");
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 10);
    }
}
