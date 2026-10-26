//! # 2016 day 25 - Clock Signal
//!
//! Yet another Assembunny problem, with yet another opcode.
//!
//! I could have re-used the common assembunny module, but the new input program uses a really
//! ineffective way of performing integer division, so it would require implementing yet
//! another optimization block which would effectively replace most of the program, so at this
//! point I might as well extract the interesting numbers from the input and perform the
//! corresponding calculations, rather than running the emulator.
//!
//! The program is actually taking the initial value of register a and creating a start-number with
//! `n = a + (b * c)`, where the initial register values for `b`and `c` are given on lines 2-3 in
//! the input program. The resulting number is then right-shifted repeatedly by division-by-2
//! (the costly operation mentioned above). So the output is then the binary sequence of `n + b * c`
//! repeated in reverse. The desired pattern is then found with the next highest binary number that
//! has the pattern `10101010...`.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((solution_data.solve_part1().to_string(), "-".to_string()))
}

struct InputData {
    b: u32,
    c: u32,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_number(s: &str) -> Result<u32, AocError> {
            let words: Vec<_> = s.split_ascii_whitespace().collect();
            let number = words.get(1).ctx("number")?;
            Ok(number.parse::<u32>()?)
        }
        let numbers: Vec<_> = s
            .lines()
            .skip(1)
            .take(2)
            .map(extract_number)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            b: *numbers.first().ctx("b")?,
            c: *numbers.get(1).ctx("c")?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u32 {
        let offset = self.b * self.c;
        let mut a = 0;
        // Build the alternating 101010... pattern
        while a < offset {
            a = (a << 2) | 2;
        }
        a - offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CUSTOM_1: &str = "cpy a d
cpy 3 c
cpy 5 b";

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.b, 3);
        assert_eq!(solution_data.c, 5);
        assert_eq!(solution_data.solve_part1(), 27);
    }
}
