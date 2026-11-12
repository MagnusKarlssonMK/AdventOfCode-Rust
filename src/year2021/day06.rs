//! # 2021 day 6 - Lanternfish
//!
//! After each day every timer decreases by one, which is equivalent to reading the same array
//! starting one slot later. When a fish reaches timer 0 it must also spawn a newborn at timer 8.
//! Instead of moving counts around, the current timer-0 bucket is added to the bucket that will
//! hold timer 6 after the slide (resetting the parent), and is left in place because after the
//! slide that same bucket holds timer 8 (the newborn).
//!
//! Part 1 reports the population after 80 days; Part 2 after 256 days. Both parts could have been
//! calculated in the same cycle, but it's near instant anyway so it's easier to do it separately.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

/// Time for each lantern fish to spawn a new fish.
const FISH_TIMER: usize = 7;
/// Number of lantern fish timer states (0-8).
const NBR_FISH_TIMER_STATES: usize = 9;
const P1_DAYS: usize = 80;
const P2_DAYS: usize = 256;

struct InputData {
    fish_states: [u64; NBR_FISH_TIMER_STATES],
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut states = [0u64; NBR_FISH_TIMER_STATES];
        for v in s.split(',') {
            let n = v.parse::<u64>()?;
            if n >= NBR_FISH_TIMER_STATES as u64 {
                return Err(AocError::Invalid(format!("input value too large: {n}")));
            }
            states[n as usize] += 1;
        }
        Ok(Self {
            fish_states: states,
        })
    }
}

impl InputData {
    fn simulate_fish(&self, days: usize) -> u64 {
        let mut states = self.fish_states;
        for d in 0..days {
            states[(d + FISH_TIMER) % NBR_FISH_TIMER_STATES] += states[d % NBR_FISH_TIMER_STATES]
        }
        states.into_iter().sum()
    }

    fn solve_part1(&self) -> u64 {
        self.simulate_fish(P1_DAYS)
    }

    fn solve_part2(&self) -> u64 {
        self.simulate_fish(P2_DAYS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "3,4,3,1,2";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 5934);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 26984457539);
    }

    #[test]
    fn parser_invalid_input() {
        assert!(matches!(
            "3,a,4".parse::<InputData>(),
            Err(AocError::ParseInt(_))
        ));
        assert!(matches!(
            "3,9,4".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }
}
