//! # 2023 day 14 - Parabolic Reflector Dish
use std::{collections::HashMap, error::Error, str::FromStr};

use crate::aoc_util::{grid::Grid, point::Point};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

impl Grid {
    fn tilt_north(&mut self) {
        for x in 0..self.x_max as i32 {
            let mut floor = 0;
            for y in 0..self.y_max as i32 {
                let current_point = Point::new(x, y);
                if let Some(e) = self.get_element(&current_point) {
                    match e {
                        '#' => floor = y + 1,
                        'O' => {
                            if y > floor {
                                self.set_point(&Point::new(x, floor), 'O');
                                self.set_point(&current_point, '.');
                            }
                            floor += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.x_max as i32 {
            let mut floor = self.y_max as i32 - 1;
            for y in (0..self.y_max as i32).rev() {
                let current_point = Point::new(x, y);
                if let Some(e) = self.get_element(&current_point) {
                    match e {
                        '#' => floor = y - 1,
                        'O' => {
                            if y < floor {
                                self.set_point(&Point::new(x, floor), 'O');
                                self.set_point(&current_point, '.');
                            }
                            floor -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.y_max as i32 {
            let mut floor = self.x_max as i32 - 1;
            for x in (0..self.x_max as i32).rev() {
                let current_point = Point::new(x, y);
                if let Some(e) = self.get_element(&current_point) {
                    match e {
                        '#' => floor = x - 1,
                        'O' => {
                            if x < floor {
                                self.set_point(&Point::new(floor, y), 'O');
                                self.set_point(&current_point, '.');
                            }
                            floor -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.y_max as i32 {
            let mut floor = 0;
            for x in 0..self.x_max as i32 {
                let current_point = Point::new(x, y);
                if let Some(e) = self.get_element(&current_point) {
                    match e {
                        '#' => floor = x + 1,
                        'O' => {
                            if x > floor {
                                self.set_point(&Point::new(floor, y), 'O');
                                self.set_point(&current_point, '.');
                            }
                            floor += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Performs a full tilt cycle of the dish
    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    /// Calculates the load on the north support beams
    fn get_load(&self) -> usize {
        self.elements
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 'O')
            .map(|(i, _)| self.y_max - (i / self.x_max))
            .sum()
    }
}

struct InputData {
    dish: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            dish: Grid::parse(s),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut dish = self.dish.clone();
        dish.tilt_north();
        dish.get_load()
    }

    fn solve_part2(&self) -> usize {
        let mut dish = self.dish.clone();
        let target_cycles: usize = 1_000_000_000;
        let mut seen = HashMap::new();
        let mut loads = Vec::new();

        // Keep running cycles until a previously seen state if found, storing load values along the way.
        // Then calculate the index of where we would be in the cycle when reaching the target, and pick out the load value.
        for cycle in 0..target_cycles {
            dish.cycle();
            loads.push(dish.get_load());
            if let Some(previous) = seen.insert(dish.clone(), cycle) {
                let idx = previous + ((target_cycles - 1 - previous) % (cycle - previous));
                return loads[idx];
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 136);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 64);
    }
}
