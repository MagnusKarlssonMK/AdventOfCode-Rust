//! # 2020 day 9 - Encoding Error
use crate::aoc_util::error::AocError;
use std::{cmp::Ordering, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve(25)?;
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    numbers: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self, preamble_len: usize) -> Result<(usize, usize), AocError> {
        let invalid_number = self
            .numbers
            .windows(preamble_len + 1)
            .find(|preamble| {
                !(0..preamble_len - 1).any(|i| {
                    (i + 1..preamble_len)
                        .any(|j| preamble[i] + preamble[j] == preamble[preamble_len])
                })
            })
            .map(|preamble| preamble[preamble_len])
            .ok_or(AocError::NoSolution)?;

        let mut window_sum = 0usize;
        let mut low = 0usize;
        let mut high = 0usize;
        loop {
            match window_sum.cmp(&invalid_number) {
                Ordering::Equal if high > low => break,
                Ordering::Less => {
                    let next = *self.numbers.get(high).ok_or(AocError::NoSolution)?;
                    window_sum += next;
                    high += 1;
                }
                _ => {
                    if low == high {
                        return Err(AocError::NoSolution);
                    }
                    window_sum -= self.numbers[low];
                    low += 1;
                }
            }
        }

        let window = &self.numbers[low..high];
        let min = window.iter().min().ok_or(AocError::NoSolution)?;
        let max = window.iter().max().ok_or(AocError::NoSolution)?;
        Ok((invalid_number, min + max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

    /// To verify distinct-pairs (i + 1..). Also gives a normal multi-element window
    const TEST_CUSTOM_1: &str = "5\n1\n4\n10";
    /// find() returns None -> NoSolution
    const TEST_CUSTOM_2: &str = "1\n2\n3";
    /// Empty-window guard (invalid number 0)
    const TEST_CUSTOM_3: &str = "1\n2\n0";
    /// Short input -> NoSolution
    const TEST_CUSTOM_4: &str = "1\n2";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve(5).unwrap();
        assert_eq!(p1, 127);
        assert_eq!(p2, 62);
    }

    #[test]
    fn part1_2_custom_1() {
        let data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(data.solve(3).unwrap(), (10, 6));
    }

    #[test]
    fn no_invalid_number() {
        let data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(data.solve(2).unwrap_err(), AocError::NoSolution);
    }

    #[test]
    fn empty_window() {
        let data: InputData = TEST_CUSTOM_3.parse().unwrap();
        assert_eq!(data.solve(2).unwrap_err(), AocError::NoSolution);
    }

    #[test]
    fn too_short() {
        let data: InputData = TEST_CUSTOM_4.parse().unwrap();
        assert_eq!(data.solve(2).unwrap_err(), AocError::NoSolution);
    }
}
