//! # 2025 day 3 - Lobby
//!
//! A simple solution is to scan each row ('bank') left to right, finding the value and position of
//! the largest number (first from lefthand side if duplicates), then start from that position and
//! scan for the next digit until the needed number of digits have been found.
//!
//! However there are quite few nines in the input, so that approach tends to make us scan each row
//! almost completely multiple times. So as a generally faster approach:
//!
//! 1. Assign the L wanted digits from the L rightmost digits in the row with in total N digits.
//! 2. Then start sweeping the remaining N-L digits right to left.
//! 3. For each digit, compare it to the first assigned digit; if the new digit is >= the stored
//!    one, swap them, and then go to the next stored digit and repeat until stored digit > new
//!    digit and then go back to 2 and repeat until the whole row has been scanned.
//!
//! This means we only sweep the line once, and pretty much the only way the basic algorithm would
//! be faster is if there are L nines in the row. We could also break early with this approach and
//! check whether the first and last value are both 9, but that seems to cost more execution time
//! than it saves. Validation is deliberately fused into that same sweep: a separate pass that
//! converts the whole input to characters first was measurably slower.
use crate::aoc_util::error::{AocError, OptionExt};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData<'a> {
    banks: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            banks: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn get_jolt(&self, nbrof_digits: usize) -> Result<u64, AocError> {
        let mut total = 0;
        for bank in self.banks.iter() {
            let mut result = Vec::with_capacity(nbrof_digits);
            let mut bytes = bank.bytes().rev();
            for _ in 0..nbrof_digits {
                let b = bytes.next().ctx("bank has too few digits")?;
                if !b.is_ascii_digit() {
                    return Err(AocError::Invalid(format!(
                        "invalid character: {}",
                        char::from(b)
                    )));
                }
                result.push(b);
            }
            result.reverse();
            for i in bytes {
                if !i.is_ascii_digit() {
                    return Err(AocError::Invalid(format!(
                        "invalid character: {}",
                        char::from(i)
                    )));
                }
                let mut x = i;
                for j in result.iter_mut() {
                    if x >= *j {
                        std::mem::swap(&mut x, &mut *j);
                    } else {
                        break;
                    }
                }
            }
            total += result
                .iter()
                .rev()
                .enumerate()
                .map(|(i, v)| 10u64.pow(i as u32) * (*v - b'0') as u64)
                .sum::<u64>();
        }
        Ok(total)
    }

    fn solve_part1(&self) -> Result<u64, AocError> {
        self.get_jolt(2)
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        self.get_jolt(12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 357);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 3121910778619);
    }

    #[test]
    fn invalid_banks() {
        for bank in ["12x456789012", "12345678901x", "9/9", "\n"] {
            let solution_data = InputData::try_from(bank).unwrap();
            assert!(
                matches!(solution_data.solve_part1(), Err(AocError::Invalid(_))),
                "expected Invalid for {bank:?}"
            );
        }
    }

    #[test]
    fn short_bank() {
        let solution_data = InputData::try_from("123").unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 23);
        assert!(matches!(
            solution_data.solve_part2(),
            Err(AocError::Invalid(_))
        ));
    }
}
