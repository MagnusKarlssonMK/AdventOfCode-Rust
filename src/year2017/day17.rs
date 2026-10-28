//! # 2017 day 17 - Spinlock
//!
//! ## Part 1
//!
//! Just build the buffer while keeping track of the current position.
//!
//! ## Part 2
//!
//! The value 0 never moves from index 0, so we only need to know which value ends up at index 1. A naive
//! loop over all 50M insertions is slow, so whole stretches are skipped at a time:
//!
//! Between two wrap-arounds the position simply climbs. Each insertion moves the current position by `steps`
//! positions plus one extra step for the newly inserted value (steps + 1 in total), while the buffer only
//! grows by one. The gap between the current position and the end of the buffer therefore shrinks by exactly
//! `steps` every insertion, and no value can land at index 1 while the position is just climbing. So the climb
//! up to the next wrap-around can be jumped over in one go: the jump is `(gap - 1) // steps` rounds (a
//! ceil-div). At a wrap-around the position drops back below `steps`, so the number of wraps - and thus the
//! number of loop iterations - is tiny.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2(PART_2_TARGET).to_string(),
    ))
}

const PART_1_TARGET: usize = 2017;
const PART_2_TARGET: usize = 50_000_000;

struct InputData {
    steps: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { steps: s.parse()? })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut buffer = Vec::from([0usize]);
        let mut current_pos = 0;
        for nbr in 1..=PART_1_TARGET {
            current_pos = 1 + (current_pos + self.steps) % nbr;
            buffer.insert(current_pos, nbr);
        }
        buffer[current_pos + 1]
    }

    fn solve_part2(&self, target: usize) -> usize {
        let mut current_pos = 0;
        let mut pos_1 = 0;
        let mut nbr = 1;
        while nbr <= target {
            let gap = nbr - current_pos;
            if gap > self.steps {
                // Climb: gap shrinks by `steps` per round, so the next wrap-around happens
                // `(gap - 1) / steps` rounds from here. No index-1 value can land in between.
                let jump = (gap - 1) / self.steps;
                if nbr + jump > target {
                    break;
                }
                nbr += jump;
                current_pos += jump * (self.steps + 1);
            }
            // Wrap-around: the only round that can drop a value at index 1.
            current_pos = 1 + (current_pos + self.steps) % nbr;
            if current_pos == 1 {
                pos_1 = nbr;
            }
            nbr += 1;
        }
        pos_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "3";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 638);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(PART_2_TARGET), 1222153);
    }
}
