//! # 2021 day 6 - Lanternfish
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    fish_states: [usize; 9],
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut states = [0_usize; 9];
        s.split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .for_each(|n| states[n] += 1);
        Ok(Self {
            fish_states: states,
        })
    }
}

impl InputData {
    fn simulate_fishies(&self, days: usize) -> usize {
        let mut states = self.fish_states;
        for d in 0..days {
            states[(d + 7) % 9] += states[d % 9]
        }
        states.iter().sum()
    }

    fn solve_part1(&self) -> usize {
        self.simulate_fishies(80)
    }

    fn solve_part2(&self) -> usize {
        self.simulate_fishies(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3,4,3,1,2";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 5934);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 26984457539);
    }
}
