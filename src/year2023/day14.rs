//! # 2023 day 14 - Parabolic Reflector Dish
//!
//! Rocks slide along rows or columns until they hit a wall or another rock. A shared `tilt_axis`
//! routine handles all four directions via const-generic axis and slide parameters; part 1 tilts
//! once north. Part 2 repeats the north/west/south/east cycle, detecting a repeated dish state to
//! skip ahead to the billionth cycle.
use crate::aoc_util::{error::AocError, grid::Grid, point::Point};
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const P2_TARGET_CYCLES: usize = 1_000_000_000;

impl Grid<char> {
    fn tilt_north(&mut self) {
        self.tilt_axis::<true, -1>();
    }

    fn tilt_south(&mut self) {
        self.tilt_axis::<true, 1>();
    }

    fn tilt_east(&mut self) {
        self.tilt_axis::<false, 1>();
    }

    fn tilt_west(&mut self) {
        self.tilt_axis::<false, -1>();
    }

    /// Slides every 'O' along each row or column toward the destination side. `VERTICAL` selects
    /// the axis and `STEP` is -1 for north/west (toward coordinate 0) or +1 for south/east. Making
    /// them const generics gives each direction its own specialised loop with no per-cell branches.
    fn tilt_axis<const VERTICAL: bool, const STEP: i32>(&mut self) {
        let (outer_len, inner_len) = if VERTICAL {
            (self.x_max, self.y_max)
        } else {
            (self.y_max, self.x_max)
        };
        for outer in 0..outer_len as i32 {
            // The next free slot on the destination side, advanced past rocks and round walls.
            let mut floor = if STEP > 0 { inner_len as i32 - 1 } else { 0 };
            for i in 0..inner_len as i32 {
                // Scan from the destination side inward.
                let inner = if STEP > 0 { inner_len as i32 - 1 - i } else { i };
                let point = if VERTICAL {
                    Point::new(outer, inner)
                } else {
                    Point::new(inner, outer)
                };
                // The explicit comparison and `if let` keep this loop scalar; a `let ... else`
                // with arithmetic here gets auto-vectorized and runs about 3x slower.
                if let Some(c) = self.get_ref(&point) {
                    match c {
                        '#' => floor = inner - STEP,
                        'O' => {
                            let needs_move = if STEP < 0 {
                                inner > floor
                            } else {
                                inner < floor
                            };
                            if needs_move {
                                let dest = if VERTICAL {
                                    Point::new(outer, floor)
                                } else {
                                    Point::new(floor, outer)
                                };
                                self.set_point(&dest, 'O');
                                self.set_point(&point, '.');
                            }
                            floor -= STEP;
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
    dish: Grid<char>,
}

impl FromStr for InputData {
    type Err = AocError;
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

    fn solve_part2(&self) -> Result<usize, AocError> {
        let mut dish = self.dish.clone();
        let mut seen = HashMap::new();
        let mut loads = Vec::new();

        // Keep running cycles until a previously seen state if found, storing load values along the way.
        // Then calculate the index of where we would be in the cycle when reaching the target, and pick out the load value.
        for cycle in 0..P2_TARGET_CYCLES {
            dish.cycle();
            loads.push(dish.get_load());
            if let Some(previous) = seen.insert(dish.clone(), cycle) {
                let idx = previous + ((P2_TARGET_CYCLES - 1 - previous) % (cycle - previous));
                return Ok(loads[idx]);
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "O....#....
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
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 136);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 64);
    }
}
