//! # 2025 day 3 - Lobby
//!
//! ## Part 1
//!
//! * Scan each row ("bank") excluding the last character (to make room for the second digit).
//!   Iterate character by character, from left to right, to find the value and position of the
//!   max number. Stop at the first 9 that is found, if any.
//! * Scan again (now including the last character), starting from the position after where
//!   the first digit was found, and similarly find the largest second digit.
//! * Combine the two digits to get the jolt value.
//!
//! ## Part 2
//!
//! Same as part 1, except repeat the search for digits 12 times instead of just 2.
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    banks: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            banks: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn get_jolt(&self, nbrof_digits: usize) -> usize {
        let mut total = 0;
        for bank in self.banks.iter() {
            let mut new_number = 0;
            let mut start_idx = 0;
            for digit_idx in (0..nbrof_digits).rev() {
                let mut next_digit = 0;
                let i = start_idx;
                for x in i..bank.len() - digit_idx {
                    let new_digit: u32 = bank.get(x..x + 1).unwrap().parse().unwrap();
                    if next_digit < new_digit {
                        next_digit = new_digit;
                        start_idx = x + 1;
                        if next_digit == 9 {
                            break;
                        }
                    }
                }
                new_number += next_digit as usize * 10_usize.pow(digit_idx as u32);
            }
            total += new_number;
        }
        total
    }

    fn solve_part1(&self) -> usize {
        self.get_jolt(2)
    }

    fn solve_part2(&self) -> usize {
        self.get_jolt(12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 357);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 3121910778619);
    }
}
