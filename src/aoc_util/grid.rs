//! # Grid utility
//!
//! Provides a struct for managing a 2D grid of elements of type char,
//! stored internally in a one-dimensional vector.
use core::fmt;

use crate::aoc_util::point::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub x_max: usize,
    pub y_max: usize,
    pub elements: Vec<char>,
}

impl Grid {
    #[inline]
    /// Creates a new `Grid` instance based on a string input.
    ///
    /// Every line is padded with spaces to the width of the widest line, so the
    /// grid is always rectangular. Input files whose trailing spaces have been
    /// stripped would otherwise produce a ragged grid, where the element vector
    /// ends up shorter than `x_max * y_max` and every row after the short one
    /// is read from the wrong offset.
    pub fn parse(input: &str) -> Self {
        let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let x_max = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let y_max = lines.len();

        let mut elements = Vec::with_capacity(x_max * y_max);
        for line in &lines {
            elements.extend_from_slice(line);
            // Pad the row out to the full width.
            elements.resize(elements.len() + (x_max - line.len()), ' ');
        }

        Self {
            x_max,
            y_max,
            elements,
        }
    }

    /// Creates a new `Grid` instance of the specified dimensions `x_max` : `y_max` and initiated to the
    /// character in `val`.
    pub fn new(x_max: usize, y_max: usize, val: char) -> Self {
        Self {
            x_max,
            y_max,
            elements: vec![val; x_max * y_max],
        }
    }

    /// Returns the value of the element in point `p`. Returns `None` if the point doesn't exist in the grid.
    #[inline]
    pub fn get_element(&self, p: &Point) -> Option<char> {
        if (0..self.x_max).contains(&(p.x as usize)) && (0..self.y_max).contains(&(p.y as usize)) {
            Some(self.elements[self.x_max * (p.y as usize) + (p.x as usize)])
        } else {
            None
        }
    }

    /// Returns the value of the element in point `p`, translated to an u8 number. Returns `None` if the point
    /// doesn't exist in the grid or if the element is not a digit.
    #[inline]
    pub fn get_uint_element(&self, p: &Point) -> Option<u8> {
        if (0..self.x_max).contains(&(p.x as usize)) && (0..self.y_max).contains(&(p.y as usize)) {
            self.elements[self.x_max * (p.y as usize) + (p.x as usize)]
                .to_digit(10)
                .map(|n| n as u8)
        } else {
            None
        }
    }

    /// Returns the first position in the grid containing the character `item`. Returns `None` if the character
    /// is not found in the grid.
    #[inline]
    pub fn find(&self, item: char) -> Option<Point> {
        self.elements
            .iter()
            .position(|&c| c == item)
            .map(|i| Point::new((i % self.x_max) as i32, (i / self.x_max) as i32))
    }

    /// Returns the index in the element array of the point `p`.
    /// TBD - change to return Option and None if the point is outside the grid.
    #[inline]
    pub fn get_index(&self, p: &Point) -> usize {
        self.x_max * p.y as usize + p.x as usize
    }

    /// Returns the point of the `index` of the element array.
    /// TBD - change to return Option and None if the index is outside the array.
    #[inline]
    pub fn get_point(&self, index: usize) -> Point {
        Point::new((index % self.x_max) as i32, (index / self.x_max) as i32)
    }

    /// Sets the character in point `p` to the character `val`.
    #[inline]
    pub fn set_point(&mut self, p: &Point, val: char) {
        if let Some(e) = self
            .elements
            .get_mut(self.x_max * p.y as usize + p.x as usize)
        {
            *e = val;
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = Vec::with_capacity(self.elements.len() + self.y_max);
        for y in 0..self.y_max {
            for x in 0..self.x_max {
                out.push(self.elements[y * self.x_max + x]);
            }
            out.push('\n');
        }
        write!(f, "{}", out.iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_uniform_grid() {
        let grid = Grid::parse("abc\ndef");
        assert_eq!(grid.x_max, 3);
        assert_eq!(grid.y_max, 2);
        assert_eq!(grid.elements, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn parse_pads_short_lines() {
        // Lines are padded with spaces to the width of the widest line, so the
        // grid stays rectangular even if trailing spaces were stripped.
        let grid = Grid::parse("abcd\na\nbc");
        assert_eq!(grid.x_max, 4);
        assert_eq!(grid.y_max, 3);
        assert_eq!(
            grid.elements,
            vec!['a', 'b', 'c', 'd', 'a', ' ', ' ', ' ', 'b', 'c', ' ', ' ']
        );
        assert_eq!(grid.get_element(&Point::new(0, 1)), Some('a'));
        assert_eq!(grid.get_element(&Point::new(3, 1)), Some(' '));
        assert_eq!(grid.get_element(&Point::new(1, 2)), Some('c'));
    }

    #[test]
    fn parse_empty_input() {
        let grid = Grid::parse("");
        assert_eq!(grid.x_max, 0);
        assert_eq!(grid.y_max, 0);
        assert!(grid.elements.is_empty());
    }
}
