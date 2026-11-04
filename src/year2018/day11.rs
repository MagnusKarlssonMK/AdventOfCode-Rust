//! # 2018 day 11 - Chronal Charge
//!
//! Using `summed-area table` algorithm. (https://en.wikipedia.org/wiki/Summed-area_table)
use crate::aoc_util::{error::*, grid::Grid, point::Point};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((solution_data.solve_part1(), solution_data.solve_part2()))
}

const GRID_SIZE: usize = 300;

struct InputData {
    power_levels: Grid<i32>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let serial_nbr: i32 = s.parse()?;
        let mut power_levels: Grid<i32> = Grid::new(GRID_SIZE + 2, GRID_SIZE + 2, &0);
        for x in 1..=GRID_SIZE as i32 {
            for y in 1..=GRID_SIZE as i32 {
                let rack_id = x + 10;
                let power = (rack_id * y + serial_nbr) * rack_id;
                let power = (power / 100) % 10 - 5;
                let power = power
                    + power_levels.get_element(&Point::new(x, y - 1)).unwrap()
                    + power_levels.get_element(&Point::new(x - 1, y)).unwrap()
                    - power_levels.get_element(&Point::new(x - 1, y - 1)).unwrap();
                power_levels.set_point(&Point::new(x, y), power);
            }
        }
        Ok(Self { power_levels })
    }
}

impl InputData {
    /// (x, y, power)
    fn get_best_fuel_cell(&self, fuelcell_size: i32) -> (i32, i32, i32) {
        let mut max_power = 0;
        let mut max_fuelcell_x = 0;
        let mut max_fuelcell_y = 0;
        let d = fuelcell_size - 1;
        let pairs: Vec<_> = (1..=GRID_SIZE as i32 - fuelcell_size)
            .map(|x| (x - 1, x + d))
            .collect();

        for y in 1..GRID_SIZE as i32 - fuelcell_size {
            let row_lo: Vec<_> = (0..self.power_levels.x_max as i32)
                .map(|x| {
                    self.power_levels
                        .get_element(&Point::new(x, y - 1))
                        .unwrap()
                })
                .collect();
            let row_hi: Vec<_> = (0..self.power_levels.x_max as i32)
                .map(|x| {
                    self.power_levels
                        .get_element(&Point::new(x, y + d))
                        .unwrap()
                })
                .collect();
            for (x, (x_lo, x_hi)) in pairs.iter().enumerate() {
                let power = row_lo[*x_lo as usize] + row_hi[*x_hi as usize]
                    - row_hi[*x_lo as usize]
                    - row_lo[*x_hi as usize];
                if power > max_power {
                    max_power = power;
                    max_fuelcell_x = x as i32 + 1;
                    max_fuelcell_y = y;
                }
            }
        }
        (max_fuelcell_x, max_fuelcell_y, max_power)
    }

    fn solve_part1(&self) -> String {
        let (x, y, _) = self.get_best_fuel_cell(3);
        format!("{x},{y}")
    }

    fn solve_part2(&self) -> String {
        let mut max_power = 0;
        let mut max_fuelcell_x = 0;
        let mut max_fuelcell_y = 0;
        let mut max_fuelcell_size = 0;
        for size in 1..=GRID_SIZE as i32 {
            let (x, y, power) = self.get_best_fuel_cell(size);
            if power > max_power {
                max_power = power;
                (max_fuelcell_x, max_fuelcell_y, max_fuelcell_size) = (x, y, size);
            }
        }
        format!("{max_fuelcell_x},{max_fuelcell_y},{max_fuelcell_size}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "18";
    const TEST_OFFICIAL_2: &str = "42";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), "33,45");
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1(), "21,61");
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), "90,269,16");
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), "232,251,12");
    }
}
