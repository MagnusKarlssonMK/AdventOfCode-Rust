//! # 2021 day 7 - The Treachery of Whales
//!
//! Part 1 is minimized by the median (sum of absolute deviations), part 2 by the mean because the
//! cost is triangular/convex, hence checking the floor mean and its neighbours.
use crate::aoc_util::error::AocError;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    crabs: Vec<u32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            crabs: s.split(',').map(|n| n.parse()).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn get_median(&self) -> u32 {
        let mut crabs = self.crabs.to_vec();
        crabs.sort_unstable();
        let middle = crabs.len() / 2;
        if crabs.len().is_multiple_of(2) {
            crabs[middle - 1].midpoint(crabs[middle])
        } else {
            crabs[middle]
        }
    }

    fn get_mean(&self) -> u32 {
        let sum: u64 = self.crabs.iter().map(|&c| u64::from(c)).sum();
        (sum / self.crabs.len() as u64) as u32
    }

    fn get_scaling_cost(&self, cal_nbr: u32) -> u64 {
        let scale = |x: u64| x * (x + 1) / 2;
        self.crabs
            .iter()
            .map(|crab| scale(crab.abs_diff(cal_nbr) as u64))
            .sum()
    }

    fn solve_part1(&self) -> u64 {
        let cal_nbr = self.get_median();
        self.crabs
            .iter()
            .map(|crab| u64::from(crab.abs_diff(cal_nbr)))
            .sum()
    }

    fn solve_part2(&self) -> u64 {
        let distance = self.get_mean();
        self.get_scaling_cost(distance.saturating_sub(1)).min(
            self.get_scaling_cost(distance)
                .min(self.get_scaling_cost(distance.saturating_add(1))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "16,1,2,0,4,2,7,1,2,14";

    const TEST_CUSTOM_1: &str = "0,1,2,3,4,5,7";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 37);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 168);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
        assert_eq!(solution_data.solve_part2(), 24);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data: InputData = "0,0,1".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
        assert_eq!(solution_data.solve_part2(), 1);
    }
}
