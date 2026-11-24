//! # 2023 day 1 - Trebuchet?!
//!
//! Part 1 takes the first and last digit on each line. Part 2 also counts spelled-out digits, and
//! checks every position rather than skipping past a match, so overlapping spellings such as
//! `eightwo` contribute eight as the first digit and two as the last.
use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const SPELLED_OUT: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

struct InputData<'a> {
    calibration_lines: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            calibration_lines: s.lines().collect(),
        })
    }
}

fn calculate_line(line: &str, spelled_out: bool) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut first = None;
    let mut last = None;

    for (i, &byte) in bytes.iter().enumerate() {
        let value = if byte.is_ascii_digit() {
            Some((byte - b'0') as usize)
        } else if spelled_out {
            SPELLED_OUT
                .iter()
                .position(|word| bytes[i..].starts_with(word))
                .map(|n| n + 1)
        } else {
            None
        };
        if let Some(value) = value {
            first.get_or_insert(value);
            last = Some(value);
        }
    }
    Some(first? * 10 + last?)
}

impl InputData<'_> {
    fn solve(&self, spelled_out: bool) -> Result<usize, AocError> {
        self.calibration_lines
            .iter()
            .map(|line| {
                calculate_line(line, spelled_out).ok_or_else(|| {
                    AocError::Invalid(format!("no calibration value in line: {line:?}"))
                })
            })
            .sum()
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        self.solve(false)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        self.solve(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_OFFICIAL_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    const TEST_CUSTOM_1: &str = "trebuchet";

    const TEST_CUSTOM_2: &str = "abcdfiveefg";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 142);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert!(solution_data.solve_part1().is_err());
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 281);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert!(solution_data.solve_part2().is_err());
    }

    #[test]
    fn part2_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        // A line with only a spelled-out digit is valid for part 2 but not part 1.
        assert!(solution_data.solve_part1().is_err());
        assert_eq!(solution_data.solve_part2().unwrap(), 55);
    }
}
