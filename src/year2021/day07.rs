//! # 2021 day 7 - The Treachery of Whales
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    crabs: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            crabs: s.split(',').map(|n| n.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn get_median(&self) -> usize {
        let mut crabs = self.crabs.to_vec();
        crabs.sort_unstable();
        let middle = crabs.len() / 2;
        if crabs.len() % 2 == 0 {
            (crabs[middle - 1] + crabs[middle]) / 2
        } else {
            crabs[middle]
        }
    }

    fn get_mean(&self) -> usize {
        self.crabs.iter().sum::<usize>() / self.crabs.len()
    }

    fn get_scaling_cost(&self, cal_nbr: usize) -> usize {
        let scale = |x: usize| x * (x + 1) / 2;
        self.crabs
            .iter()
            .map(|crab| scale((*crab as i32 - cal_nbr as i32).unsigned_abs() as usize))
            .sum()
    }

    fn solve_part1(&self) -> usize {
        let cal_nbr = self.get_median();
        self.crabs
            .iter()
            .map(|crab| (*crab as i32 - cal_nbr as i32).unsigned_abs() as usize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let distance = self.get_mean();
        self.get_scaling_cost(distance).min(
            self.get_scaling_cost(distance - 1)
                .min(self.get_scaling_cost(distance + 1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 37);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 168);
    }
}
