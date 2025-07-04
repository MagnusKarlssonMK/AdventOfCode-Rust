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

fn calculate_line(line: &str, spelled_out: bool) -> usize {
    let mut chrline = line.as_bytes();
    let mut first = None;
    let mut last = None;

    while first.is_none() && !chrline.is_empty() {
        if chrline[0].is_ascii_digit() {
            first = Some(chrline[0].wrapping_sub(b'0') as usize);
        } else if spelled_out {
            for (i, nbr) in SPELLED_OUT.iter().enumerate() {
                if chrline.starts_with(nbr) {
                    first = Some(i + 1);
                    break;
                }
            }
        }
        if first.is_none() {
            chrline = &chrline[1..];
        }
    }

    while last.is_none() && !chrline.is_empty() {
        if chrline[chrline.len() - 1].is_ascii_digit() {
            last = Some(chrline[chrline.len() - 1].wrapping_sub(b'0') as usize);
        } else if spelled_out {
            for (i, nbr) in SPELLED_OUT.iter().enumerate() {
                if chrline.ends_with(nbr) {
                    last = Some(i + 1);
                    break;
                }
            }
        }
        if last.is_none() {
            chrline = &chrline[..chrline.len() - 1];
        }
    }
    10 * first.unwrap_or(0) + last.unwrap_or(0)
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
            .map(|line| calculate_line(line, false))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.data
            .iter()
            .map(|line| calculate_line(line, true))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        const TEST_DATA: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let solution_data = InputData::parse_input(TEST_DATA);
        assert_eq!(solution_data.solve_part1(), 142);
    }

    #[test]
    fn part1_example_2() {
        // Custom test to verify input with no numbers
        const TEST_DATA: &str = "trebuchet";
        let solution_data = InputData::parse_input(TEST_DATA);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part2_example_1() {
        const TEST_DATA: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let solution_data = InputData::parse_input(TEST_DATA);
        assert_eq!(solution_data.solve_part2(), 281);
    }

    #[test]
    fn part2_example_2() {
        // Custom test to verify input with either no numbers, or only one spelled out number
        const TEST_DATA: &str = "trebuchet
abcdfiveefg";
        let solution_data = InputData::parse_input(TEST_DATA);
        assert_eq!(solution_data.solve_part2(), 55);
    }
}
