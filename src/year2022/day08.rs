//! # 2022 day 8 - Treetop Tree House
//!
//! Every row and column is scanned from both ends, which covers the four viewing directions with
//! the same loop. A scan is described by a `ScanLine` (start index, step and length) yielding flat
//! grid indices, so the direction-dependent part is confined to `scan_lines`. During the scan, the
//! tallest tree seen so far marks visibility for part 1, while a distance table indexed by tree
//! height accumulates the four viewing distances used by part 2.
use crate::aoc_util::{error::*, grid::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

/// One straight line through the grid: a flat start index, a signed step between consecutive
/// cells, and the number of cells to visit. Walking it from the edge inwards is what lets a single
/// loop handle every direction.
#[derive(Clone, Copy)]
struct ScanLine {
    next: isize,
    step: isize,
    remaining: usize,
}

impl ScanLine {
    fn new(start: usize, step: isize, len: usize) -> Self {
        Self {
            next: start as isize,
            step,
            remaining: len,
        }
    }
}

impl Iterator for ScanLine {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.next as usize;
        self.next += self.step;
        self.remaining -= 1;
        Some(index)
    }
}

/// Yields every row left-to-right and right-to-left, then every column top-to-bottom and
/// bottom-to-top.
fn scan_lines(width: usize, height: usize) -> impl Iterator<Item = ScanLine> {
    let last_col = width.saturating_sub(1);
    let last_row = height.saturating_sub(1);
    let rows = (0..height).flat_map(move |y| {
        let start = y * width;
        [
            ScanLine::new(start, 1, width),
            ScanLine::new(start + last_col, -1, width),
        ]
    });
    let cols = (0..width).flat_map(move |x| {
        [
            ScanLine::new(x, width as isize, height),
            ScanLine::new(x + last_row * width, -(width as isize), height),
        ]
    });
    rows.chain(cols)
}

struct InputData {
    grid: Grid<u8>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s).try_map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or(AocError::Invalid(c.to_string()))
        })?;
        Ok(Self { grid })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut visible = vec![false; self.grid.elements.len()];
        let mut scenic = vec![1usize; self.grid.elements.len()];
        for line in scan_lines(self.grid.x_max, self.grid.y_max) {
            // Shorter than any tree, so the first tree of every line is always visible.
            let mut tallest = -1;
            // `distances[h]` is the viewing distance from the current cell for a viewer of height
            // `h`. Reset per line, as each direction is scanned independently.
            let mut distances = [0usize; 10];
            for idx in line {
                let height = usize::from(self.grid.elements[idx]);
                let tall = height as i8;
                if tall > tallest {
                    visible[idx] = true;
                    tallest = tall;
                }
                scenic[idx] *= distances[height];
                for (limit, distance) in distances.iter_mut().enumerate() {
                    // A viewer at least as tall as this tree is blocked by it; taller viewers see
                    // past it.
                    *distance = if limit > height { *distance + 1 } else { 1 };
                }
            }
        }
        (
            visible.iter().filter(|&&v| v).count(),
            scenic.iter().copied().max().unwrap_or(0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 21);
        assert_eq!(p2, 8);
    }

    #[test]
    fn part1_2_custom_zero_height_edge() {
        let solution_data: InputData = "000\n000\n000".parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 8);
        assert_eq!(p2, 1);
    }

    #[test]
    fn part1_2_custom_single_column() {
        let solution_data: InputData = "1\n2\n3".parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3);
        assert_eq!(p2, 0);
    }

    #[test]
    fn parser_invalid_input() {
        assert!("3037a".parse::<InputData>().is_err());
    }
}
