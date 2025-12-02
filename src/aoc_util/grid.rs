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
    /// The function assumes that the input is correct, i.e. non-empty string and the number
    /// of characters in each line is consistent.
    pub fn parse(input: &str) -> Self {
        let lines: Vec<_> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let x_max = lines[0].len();
        let y_max = lines.len();
        let mut elements = Vec::with_capacity(x_max * y_max);
        lines
            .iter()
            .for_each(|line| line.iter().for_each(|c| elements.push(*c)));
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
