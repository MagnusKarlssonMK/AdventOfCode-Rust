//! # 2023 day 1 - Trebuchet?!
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::parse_input(input);
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const SPELLED_OUT: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

struct InputData<'a> {
    data: Vec<&'a str>,
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self {
            data: input.lines().collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        self.data
            .iter()
            .map(|line| {
                let first = line
                    .chars()
                    .find(char::is_ascii_digit)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let last = line
                    .chars()
                    .rfind(char::is_ascii_digit)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                (10 * first + last) as usize
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.data
            .iter()
            .map(|line| {
                let mut chrline = line.as_bytes();
                let mut first: Option<usize> = None;
                let mut last: Option<usize> = None;

                while first.is_none() {
                    if chrline[0].is_ascii_digit() {
                        first = Some(chrline[0].wrapping_sub(b'0') as usize);
                        break;
                    }
                    for (i, nbr) in SPELLED_OUT.iter().enumerate() {
                        if chrline.starts_with(nbr) {
                            first = Some(i + 1);
                            break;
                        }
                    }
                    chrline = &chrline[1..];
                }

                while last.is_none() {
                    if chrline[chrline.len() - 1].is_ascii_digit() {
                        last = Some(chrline[chrline.len() - 1].wrapping_sub(b'0') as usize);
                        break;
                    }
                    for (i, nbr) in SPELLED_OUT.iter().enumerate() {
                        if chrline.ends_with(nbr) {
                            last = Some(i + 1);
                            break;
                        }
                    }
                    chrline = &chrline[..chrline.len() - 1];
                }
                10 * first.unwrap() + last.unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let solution_data = InputData::parse_input(testdata);
        assert_eq!(solution_data.solve_part1(), 142);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let solution_data = InputData::parse_input(testdata);
        assert_eq!(solution_data.solve_part2(), 281);
    }
}
