//! # 2021 day 11 - Dumbo Octopus
//!
//! Simulates each step by increasing every octopus once, propagating a flash to its eight
//! neighbours, and recording which cells have already flashed so each is counted at most once.
//! Part 1 sums flashes over 100 steps; part 2 finds the first step where every octopus flashes.
use crate::aoc_util::{
    error::AocError,
    grid::Grid,
    point::{NEIGHBORS_ALL, Point},
};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

const P1_NBR_STEPS: usize = 100;

impl Grid<u8> {
    fn take_step(&mut self) -> usize {
        let mut flashed = Grid::new(self.x_max, self.y_max, &false);
        for y in 0..self.y_max {
            for x in 0..self.x_max {
                let p = Point::new(x as i32, y as i32);
                self.increment(&p, &mut flashed);
            }
        }
        flashed.elements.iter().filter(|v| **v).count()
    }

    fn increment(&mut self, p: &Point, flashed: &mut Grid<bool>) {
        if flashed.get_element(p) == Some(true) {
            return;
        }
        let Some(v) = self.get_element(p) else {
            return;
        };
        if v < 9 {
            self.set_point(p, v + 1);
        } else {
            self.set_point(p, 0);
            flashed.set_point(p, true);
            for adjacent in NEIGHBORS_ALL.iter().map(|d| *p + *d) {
                self.increment(&adjacent, flashed);
            }
        }
    }
}

struct InputData {
    grid: Grid<u8>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::parse(s).try_map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or_else(|| AocError::Invalid(format!("invalid energy level: {c:?}")))
            })?,
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut octogrid = self.grid.clone();
        let max_flashes = octogrid.x_max * octogrid.y_max;
        let mut p1 = 0;
        let mut p2 = 0;
        let mut step_count = 0;
        while p2 == 0 || step_count < P1_NBR_STEPS {
            step_count += 1;
            let flashes = octogrid.take_step();
            if step_count <= P1_NBR_STEPS {
                p1 += flashes;
            }
            if p2 == 0 && flashes == max_flashes {
                p2 = step_count;
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const TEST_CUSTOM_1: &str = "567
789
123";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1656);
        assert_eq!(p2, 195);
    }

    #[test]
    fn parts1_2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 110);
        assert_eq!(p2, 61);
    }

    #[test]
    fn parser_invalid_input() {
        let err = "12a\n345".parse::<InputData>().err().unwrap();
        assert_eq!(
            err,
            AocError::Invalid("invalid energy level: 'a'".to_string())
        );
    }
}
