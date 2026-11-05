//! # 2018 day 12 - Subterranean Sustainability
//!
//! 1D game of life kind of. Store the pot state as a set of indices containing a plant, to make
//! it a bit easier to deal with the growing sides, and temporarily convert it to string while
//! creating a new generation.
//!
//! The solution represents ('#', '.') as binary (1, 0). Since each node looks at two other nodes
//! in each direction, the spread map becomes 5-bit numbers (0-31). Meaning that there are 32
//! possible spread combinations. So the entire spread pattern can actually be stored as a single
//! 32-bit bitmask.
//!
//! As showed in the example in the puzzle description, we don't actually need the spread patterns
//! that maps to '.' since that's implicit by initializing everything to 0, so we can skip parsing
//! those.
//!
//! Small note: the conversion mapping is essentially a bitmap of 5 bits, i.e. 32 diverent
//! combinations, which includes "....." (or all zeroes). I assume that this value will always
//! map to "." in the input, otherwise it would in theory turn on an infinite amount of positions
//! in both directions when taking the first step. The implementation only ever checks two
//! positions outside the ones at the edges.
//!
//! ## Part 1
//!
//! Simply generate 20 times and take the sum of the set of pots.
//!
//! ## Part 2
//!
//! The sum growth for each generation will eventually converge to a constant value, so keep
//! looping new generations and checking how much it increases, and once it seems to have settled
//! (using 20 times repeated delta here), we can use that to calculate the final value.
//!
//! Simulating 50 billion generations will obviously not be possible to brute-force, not only would
//! it take forever but it would also blow up RAM since we keep the ever growing state in memory. So
//! to be safe against any potential input that would never reach the steady state, a hard upper
//! limit is added for max number of cycles before declaring NoSolution. Steady state seems to be
//! reached in a few hundred cycles, so an upper limit of 10_000 should be on the safe side.
use crate::aoc_util::error::*;
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const P2_TARGET_CYCLES: isize = 50_000_000_000;
const P2_STEADY_STATE: usize = 20;
const P2_WATCHDOG_LIMIT: isize = 10_000;

struct InputData {
    initial_state: Vec<u8>,
    spread: u32,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn map_spread_line(s: &str) -> Result<(u32, bool), AocError> {
            match s.split(" => ").collect::<Vec<_>>().as_slice() {
                [left, "#"] => {
                    let x = left
                        .chars()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
                        .sum();
                    if x < 32 {
                        Ok((x, true))
                    } else {
                        Err(AocError::Invalid(s.to_string()))
                    }
                }
                [_, "."] => {
                    // Don't bother parsing the left side when it maps to '.' / zero, that's default anyway
                    Ok((0, false))
                }
                _ => Err(AocError::Invalid(s.to_string())),
            }
        }

        let (state_str, spread_str) = s.split_once("\n\n").ctx("block separator")?;
        let initial_state: Vec<_> = state_str
            .trim_start_matches("initial state: ")
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect();
        let mut spread = 0;
        for line in spread_str.lines() {
            let (v, is_one) = map_spread_line(line)?;
            if is_one {
                spread += 1 << v;
            }
        }
        Ok(Self {
            initial_state,
            spread,
        })
    }
}

impl InputData {
    fn generate(&self, state: &HashSet<isize>) -> HashSet<isize> {
        let mut next = HashSet::new();
        if state.is_empty() {
            return next;
        }
        let start = state.iter().min().unwrap() - 2;
        let stop = state.iter().max().unwrap() + 3;
        let mut mask = 0usize;
        for i in start..stop {
            mask = (mask << 1) & 0b11111;
            if state.contains(&(i + 2)) {
                mask += 1;
            }
            if (self.spread >> mask) & 1 == 1 {
                next.insert(i);
            }
        }
        next
    }

    fn solve_part1(&self) -> isize {
        let mut state: HashSet<_> = self
            .initial_state
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == 1)
            .map(|(i, _)| i as isize)
            .collect();
        for _ in 0..20 {
            state = self.generate(&state);
        }
        state.iter().sum()
    }

    fn solve_part2(&self) -> Result<isize, AocError> {
        let mut state: HashSet<_> = self
            .initial_state
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == 1)
            .map(|(i, _)| i as isize)
            .collect();
        let mut generations = 0;
        let mut delta = 0;
        let mut delta_count = 0;
        while delta_count < P2_STEADY_STATE {
            let state_sum: isize = state.iter().sum();
            state = self.generate(&state);
            let new_state_sum: isize = state.iter().sum();
            if new_state_sum - state_sum == delta {
                delta_count += 1;
            } else {
                delta = new_state_sum - state_sum;
                delta_count = 0;
            }
            generations += 1;
            if generations > P2_WATCHDOG_LIMIT {
                return Err(AocError::NoSolution);
            }
        }
        println!("generations: {generations}");
        Ok(state.iter().sum::<isize>() + delta * (P2_TARGET_CYCLES - generations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 325);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 999999999374);
    }
}
