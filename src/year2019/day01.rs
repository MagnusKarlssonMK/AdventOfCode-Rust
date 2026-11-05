//! # 2019 day 1 - The Tyranny of the Rocket Equation
use std::{error::Error, str::FromStr};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

fn calc_mass(mass: usize, countfuel: bool) -> usize {
    let fuel_own_mass = (mass / 3).saturating_sub(2);
    if fuel_own_mass == 0 || !countfuel {
        fuel_own_mass
    } else {
        fuel_own_mass + calc_mass(fuel_own_mass, countfuel)
    }
}

struct InputData {
    masses: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            masses: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.masses.iter().map(|m| calc_mass(*m, false)).sum()
    }

    fn solve_part2(&self) -> usize {
        self.masses.iter().map(|m| calc_mass(*m, true)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = "12".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = "14".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = "1969".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 654);
    }

    #[test]
    fn part1_official_4() {
        let solution_data: InputData = "100756".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 33583);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = "14".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = "1969".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 966);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = "100756".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 50346);
    }
}
