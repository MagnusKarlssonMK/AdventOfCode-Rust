//! # 2017 day 9 - Stream Processing
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    let (p1, p2) = solution_data.solve_parts_1_2();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData<'a> {
    stream: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { stream: s })
    }
}

impl InputData<'_> {
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
                            '!' => {
                                tokens.next();
                            }
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
                _ => (),
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
        let testdata = "{}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 1);
    }

    #[test]
    fn part_1_example_2() {
        let testdata = "{{{}}}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 6);
    }

    #[test]
    fn part_1_example_3() {
        let testdata = "{{},{}}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 5);
    }

    #[test]
    fn part_1_example_4() {
        let testdata = "{{{},{},{{}}}}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 16);
    }

    #[test]
    fn part_1_example_5() {
        let testdata = "{<a>,<a>,<a>,<a>}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 1);
    }

    #[test]
    fn part_1_example_6() {
        let testdata = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 9);
    }

    #[test]
    fn part_1_example_7() {
        let testdata = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 9);
    }

    #[test]
    fn part_1_example_8() {
        let testdata = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve_parts_1_2();
        assert_eq!(p1, 3);
    }

    #[test]
    fn part_2_example_1() {
        let testdata = "<>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_example_2() {
        let testdata = "<random characters>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 17);
    }

    #[test]
    fn part_2_example_3() {
        let testdata = "<<<<>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 3);
    }

    #[test]
    fn part_2_example_4() {
        let testdata = "<{!>}>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 2);
    }

    #[test]
    fn part_2_example_5() {
        let testdata = "<!!>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_example_6() {
        let testdata = "<!!!>>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part_2_example_7() {
        let testdata = "<{o\"i!a,<{i<a>";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (_, p2) = solution_data.solve_parts_1_2();
        assert_eq!(p2, 10);
    }
}
