//! # 2019 day 1 - The Tyranny of the Rocket Equation
use std::{cmp::max, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

fn calc_mass(mass: &usize, countfuel: bool) -> usize {
    let fuel_own_mass = max(0, (*mass as i32 / 3) - 2) as usize;
    if fuel_own_mass == 0 || !countfuel {
        fuel_own_mass
    } else {
        fuel_own_mass + calc_mass(&fuel_own_mass, countfuel)
    }
}

struct InputData {
    masses: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            masses: s.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.masses.iter().map(|m| calc_mass(m, false)).sum()
    }

    fn solve_part2(&self) -> usize {
        self.masses.iter().map(|m| calc_mass(m, true)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "12";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "14";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "1969";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 654);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "100756";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 33583);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "14";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "1969";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 966);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "100756";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 50346);
    }
}
