//! # 2016 day 19 - An Elephant Named Joseph
//!
//! ## Part 1
//!
//! Straight up the Josephus problem. By using binary representation, the answer can be found by
//! shifting the most significant 1 to the end.
//!
//! ## Part 2
//!
//! Modified variant of the Josephus problem - elves steal from the elf directly across the circle
//! instead of the one on their left. It could be solved in a similar manner by converting to base-3
//! number and then doing similar modifications, but that gets quite a bit more complicated.
//! Instead, find the largest power of 3 that is still smaller than the target, and then the answer
//! can be found based on that. In short, the pattern resets with 1 as winner on every 'new' / added
//! digit in the base-3 representation (2, 4, 10, 28...); in the first half in-between those numbers,
//! the winning number is incremented by 1, while in the second half the winning number is incremented
//! by 2.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct InputData {
    number_of_elves: u64,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_of_elves: u64 = s.parse()?;
        if number_of_elves == 0 {
            return Err(AocError::Invalid("no elves".to_string()));
        }
        Ok(Self { number_of_elves })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        // Strip the most significant 1, then left shift and add the stripped 1 back as the least significant bit
        ((self.number_of_elves ^ (1 << self.number_of_elves.ilog2())) << 1) + 1
    }

    fn solve_part2(&self) -> u64 {
        if self.number_of_elves <= 2 {
            return 1;
        }
        // Find the largest power of 3 smaller than the number of elves
        let mut pow3: u64 = 1;
        while 3 * pow3 < self.number_of_elves {
            pow3 *= 3;
        }
        // If the number of elves is in the 'lower half' of the interval between base-3 values, the answer starts
        // on 1 and increases by 1; in the 'upper half' the answer increases by 2 for each number.
        if self.number_of_elves <= 2 * pow3 {
            return self.number_of_elves - pow3;
        }
        let rem = (self.number_of_elves - 1) % pow3 + 1;
        self.number_of_elves - pow3 + rem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Number of elves and the expected winners for part 1 and part 2. The
    /// expected values are the ones obtained by simulating the two rules of
    /// the puzzle, so they pin the formulas to the actual game. The five elves
    /// case is the example given in the puzzle description.
    const TEST_CASES: &[(u64, u64, u64)] = &[
        (1, 1, 1),
        (2, 1, 1),
        (3, 3, 3),
        (4, 1, 1),
        (5, 3, 2),
        (8, 1, 7),
        (9, 3, 9),
        (10, 5, 1),
        (16, 1, 7),
        (27, 23, 27),
        (28, 25, 1),
        (54, 45, 27),
        (55, 47, 29),
        (81, 35, 81),
        (82, 37, 1),
        (100, 73, 19),
    ];

    #[test]
    fn winners() {
        for &(elves, part1, part2) in TEST_CASES {
            let solution_data = InputData::from_str(&elves.to_string()).unwrap();
            assert_eq!(
                solution_data.solve_part1(),
                part1,
                "part 1 with {elves} elves"
            );
            assert_eq!(
                solution_data.solve_part2(),
                part2,
                "part 2 with {elves} elves"
            );
        }
    }

    #[test]
    fn parser_non_integer() {
        let err = InputData::from_str("23d4").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn parser_no_elves() {
        let err = InputData::from_str("0").unwrap_err();
        assert_eq!(err, AocError::Invalid("no elves".to_string()));
    }
}
