//! # 2019 day 4 - Secure Container
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct PwdNbr {
    number: usize,
}

impl FromStr for PwdNbr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            number: s.parse().unwrap(),
        })
    }
}

impl PwdNbr {
    fn from_vec(input: &[usize]) -> Self {
        Self {
            number: input
                .iter()
                .rev()
                .enumerate()
                .map(|(i, nbr)| 10usize.pow(i as u32) * nbr)
                .sum(),
        }
    }

    fn to_vec(&self) -> Vec<usize> {
        (0..=5)
            .rev()
            .map(|i| (self.number / 10usize.pow(i)) % 10)
            .collect()
    }
}

struct InputData {
    range_lower: PwdNbr,
    range_upper: PwdNbr,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.split_once('-').unwrap();
        Ok(Self {
            range_lower: PwdNbr::from_str(low).unwrap(),
            range_upper: PwdNbr::from_str(high).unwrap(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.pw_generator(false)
    }

    fn solve_part2(&self) -> usize {
        self.pw_generator(true)
    }

    fn pw_generator(&self, exactly_two: bool) -> usize {
        let mut digits = self.range_lower.to_vec();
        let mut tmp = 0;
        // Find the first valid initial value starting from 'lower' - the value never decreases
        for i in 1..digits.len() {
            if digits[i] != tmp && digits[i] < digits[i - 1] {
                digits[i] = digits[i - 1];
                tmp = digits[i];
            }
        }
        let mut current_value = PwdNbr::from_vec(&digits);
        let mut total = 0;
        while current_value.number <= self.range_upper.number {
            // Valid pwd if two adjacent digits are the same
            let mut buffer = digits[0];
            let mut counts = vec![1];
            for d in digits.iter().skip(1) {
                if *d == buffer {
                    *counts.last_mut().unwrap() += 1;
                } else {
                    counts.push(1);
                    buffer = *d;
                }
            }

            for c in &counts {
                if (!exactly_two && *c > 1) || (exactly_two && *c == 2) {
                    total += 1;
                    break;
                }
            }

            // Step the value, make sure to follow the 'never decreases' rule
            for i in (0..digits.len()).rev() {
                digits[i] += 1;
                if digits[i] <= 9 {
                    for j in i + 1..digits.len() {
                        digits[j] = digits[i];
                    }
                    break;
                }
            }
            current_value = PwdNbr::from_vec(&digits);
        }
        total
    }
}
