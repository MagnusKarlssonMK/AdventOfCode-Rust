//! # 2021 day 09 - Smoke Basin
//!
//! Create methods to find the low points and basins of the input grid. The latter uses a stripped
//! down BFS, growing from a low point over neighbouring cells until value '9' is found, since a
//! basin is an area of cells below 9 and so is always bounded by 9:s. Nothing stops two low points
//! from sharing a basin, so the flood fill records the cells it claims and a low point already
//! inside a claimed basin is skipped, rather than being counted again.
use crate::aoc_util::{
    error::AocError,
    grid::Grid,
    point::{NEIGHBORS_STRAIGHT, Point},
};
use std::{collections::VecDeque, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    grid: Grid<u8>,
    low_points: Vec<Point>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s).try_map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| AocError::Invalid(format!("invalid height: {c:?}")))
        })?;
        let low_points = grid
            .enumerate_points()
            .filter(|&(p, v)| {
                NEIGHBORS_STRAIGHT
                    .map(|d| p + d)
                    .iter()
                    .all(|n| grid.get_ref(n).is_none_or(|nv| nv > v))
            })
            .map(|(p, _)| p)
            .collect();
        Ok(Self { grid, low_points })
    }
}

impl InputData {
    fn flood_basin(&self, start: Point, claimed: &mut Grid<bool>) -> usize {
        let mut queue = VecDeque::from([start]);
        claimed.set_point(&start, true);
        let mut size = 0;
        while let Some(p) = queue.pop_front() {
            size += 1;
            for n in NEIGHBORS_STRAIGHT.map(|d| d + p) {
                if let Some(nv) = self.grid.get_ref(&n)
                    && nv < &9
                    && claimed.get_ref(&n).is_some_and(|&c| !c)
                {
                    claimed.set_point(&n, true);
                    queue.push_back(n);
                }
            }
        }
        size
    }

    fn solve_part1(&self) -> usize {
        self.low_points
            .iter()
            .map(|p| 1 + *self.grid.get_ref(p).expect("low points are on grid") as usize)
            .sum()
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let mut basin_sizes = Vec::new();
        let mut claimed = Grid::new(self.grid.x_max, self.grid.y_max, &false);
        for p in &self.low_points {
            if claimed.get_ref(p).is_some_and(|&seen| seen) {
                continue;
            }
            basin_sizes.push(self.flood_basin(*p, &mut claimed));
        }
        basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
        let [first, second, third, ..] = basin_sizes.as_slice() else {
            return Err(AocError::NoSolution);
        };
        Ok(first * second * third)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    const TEST_CUSTOM_1: &str = "9999999
2529595";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 15);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 1134);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 3);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["12a\n345", "123\n45"] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
    }

    #[test]
    fn part2_no_solution() {
        let solution_data: InputData = "909".parse().unwrap();
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }
}
