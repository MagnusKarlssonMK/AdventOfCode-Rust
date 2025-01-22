//! # 2020 day 9 - Encoding Error
use std::{cmp::Ordering, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve(25);
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    numbers: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve(&self, preamble_len: usize) -> (usize, usize) {
        let invalid_number = self
            .numbers
            .windows(preamble_len + 1)
            .find(|preamble| {
                for i in 0..preamble_len - 1 {
                    for j in i..preamble_len {
                        if preamble[i] + preamble[j] == preamble[preamble_len] {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|preamble| preamble[preamble_len])
            .unwrap();

        let mut window_sum = self.numbers[0] + self.numbers[1];
        let mut window_low = 0;
        let mut window_high = 2;
        loop {
            match window_sum.cmp(&invalid_number) {
                Ordering::Equal => break,
                Ordering::Less => {
                    window_sum += self.numbers[window_high];
                    window_high += 1;
                }
                Ordering::Greater => {
                    window_sum -= self.numbers[window_low];
                    window_low += 1;
                }
            }
        }

        (
            invalid_number,
            self.numbers[window_low..window_high].iter().min().unwrap()
                + self.numbers[window_low..window_high].iter().max().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        let (p1, p2) = solution_data.solve(5);
        assert_eq!(p1, 127);
        assert_eq!(p2, 62);
    }
}
