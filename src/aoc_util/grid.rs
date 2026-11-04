//! # Grid utility
//!
//! Provides a struct for managing a 2D grid of elements of a generic type, stored
//! internally in a one-dimensional vector.
//!
//! The core operations are generic over `T`, but with some `char`-specific helpers.
use core::fmt;

use crate::aoc_util::point::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub x_max: usize,
    pub y_max: usize,
    pub elements: Vec<T>,
}

impl Grid<char> {
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
}

impl<T> Grid<T> {
    /// Returns the flat element-array index for point `p`, or `None` if `p` is outside
    /// the grid.
    #[inline]
    fn index_of(&self, p: &Point) -> Option<usize> {
        if p.x < 0 || p.y < 0 || p.x as usize >= self.x_max || p.y as usize >= self.y_max {
            None
        } else {
            Some(self.x_max * p.y as usize + p.x as usize)
        }
    }

    /// Returns the index in the element array of the point `p`.
    ///
    /// This performs no bounds checking and assumes that `p` is inside the grid.
    /// For a checked lookup use [`get_ref`](Self::get_ref) or [`get_element`](Self::get_element).
    #[inline]
    pub fn get_index(&self, p: &Point) -> usize {
        self.x_max * p.y as usize + p.x as usize
    }

    /// Returns the point of the `index` of the element array.
    #[inline]
    pub fn get_point(&self, index: usize) -> Point {
        Point::new((index % self.x_max) as i32, (index / self.x_max) as i32)
    }

    /// Iterates over every element in row-major order, returning point- and value reference pairs.
    pub fn enumerate_points(&self) -> impl Iterator<Item = (Point, &T)> {
        self.elements
            .iter()
            .enumerate()
            .map(move |(i, v)| (self.get_point(i), v))
    }

    /// Produces a new grid of the same shape, applying `f` to every element.
    ///
    /// This is the infallible counterpart to [`try_map`](Self::try_map); use it when the conversion
    /// cannot fail, e.g. mapping `char` to a wider type or an enum with total mapping.
    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> Grid<U> {
        Grid {
            x_max: self.x_max,
            y_max: self.y_max,
            elements: self.elements.iter().map(f).collect(),
        }
    }

    /// Produces a new grid of the same shape, applying the fallible `f` to every element.
    ///
    /// Prefer this when the original grid comes from parsed external input, since an error in the
    /// input would then propagate an error rather than panicking.
    pub fn try_map<U, E>(&self, f: impl FnMut(&T) -> Result<U, E>) -> Result<Grid<U>, E> {
        let elements = self.elements.iter().map(f).collect::<Result<Vec<U>, E>>()?;
        Ok(Grid {
            x_max: self.x_max,
            y_max: self.y_max,
            elements,
        })
    }

    /// Returns a reference to the element at point `p`, or `None` if the point doesn't exist
    /// in the grid. Prefer this over [`get_element`](Self::get_element) when `T` is expensive
    /// to clone.
    #[inline]
    pub fn get_ref(&self, p: &Point) -> Option<&T> {
        self.index_of(p).map(|i| &self.elements[i])
    }

    /// Sets the character in point `p` to the character `val`, taking ownership of the value.
    ///
    /// Storing an owned value is a move (and free for `Copy` types), so callers that already hold an owned
    /// value avoid a clone. If the point is outside the grid, `val` is dropped and the grid remains unchanged.
    #[inline]
    pub fn set_point(&mut self, p: &Point, val: T) {
        if let Some(i) = self.index_of(p) {
            self.elements[i] = val;
        }
    }
}

impl<T: Clone> Grid<T> {
    /// Creates a new `Grid` instance of the specified dimensions `x_max` : `y_max`, with every element
    /// initialised to a clone of `val`.
    pub fn new(x_max: usize, y_max: usize, val: &T) -> Self {
        Self {
            x_max,
            y_max,
            elements: vec![val.clone(); x_max * y_max],
        }
    }

    /// Returns a clone of the element at point `p`. Returns `None` if the point doesn't exist in the grid.
    /// For a non-cloning lookup, see [`get_ref`](Self::get_ref).
    #[inline]
    pub fn get_element(&self, p: &Point) -> Option<T> {
        self.index_of(p).map(|i| self.elements[i].clone())
    }
}

impl<T: PartialEq> Grid<T> {
    /// Returns the first position in the grid containing an element equal to `item`. Returns `None` if
    /// no match is found.
    #[inline]
    pub fn find(&self, item: &T) -> Option<Point> {
        self.elements
            .iter()
            .position(|c| c == item)
            .map(|i| Point::new((i % self.x_max) as i32, (i / self.x_max) as i32))
    }
}

impl fmt::Display for Grid<char> {
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

    #[test]
    fn lookup_rejects() {
        let grid = Grid::parse("abc\ndef");
        // Negative coordinates returns None
        assert_eq!(grid.get_element(&Point::new(-1, 0)), None);
        assert_eq!(grid.get_element(&Point::new(0, -1)), None);
        assert_eq!(grid.get_ref(&Point::new(-1, -1)), None);
        // Coordinates past the far edge returns None
        assert_eq!(grid.get_element(&Point::new(3, 0)), None);
        assert_eq!(grid.get_element(&Point::new(0, 2)), None);
        assert_eq!(grid.get_ref(&Point::new(3, 2)), None);
        // In-bounds still work
        assert_eq!(grid.get_element(&Point::new(2, 1)), Some('f'));
        assert_eq!(grid.get_ref(&Point::new(0, 0)), Some(&'a'));
    }

    #[test]
    fn generic_integer_grid() {
        let mut grid: Grid<u32> = Grid::new(2, 2, &0);
        grid.set_point(&Point::new(1, 0), 42);
        assert_eq!(grid.get_element(&Point::new(1, 0)), Some(42));
        assert_eq!(grid.get_element(&Point::new(0, 0)), Some(0));
        assert_eq!(grid.find(&42), Some(Point::new(1, 0)));
        grid.set_point(&Point::new(5, 5), 99);
        assert_eq!(grid.find(&99), None);
    }

    #[test]
    fn enumerate_point() {
        let grid = Grid::parse("ab\ncd");
        let collected: Vec<_> = grid.enumerate_points().map(|(p, v)| (p, *v)).collect();
        assert_eq!(
            collected,
            vec![
                (Point::new(0, 0), 'a'),
                (Point::new(1, 0), 'b'),
                (Point::new(0, 1), 'c'),
                (Point::new(1, 1), 'd'),
            ]
        );
        let grid = Grid::parse("ab\ncd\nef");
        let third = grid.enumerate_points().nth(2).unwrap();
        assert_eq!(third, (Point::new(0, 1), &'c'));
    }

    #[test]
    fn map_converts_char_to_num() {
        let grid = Grid::parse("123\n456");
        let digits = grid.map(|c| c.to_digit(10).unwrap() as u8);
        assert_eq!(digits.x_max, 3);
        assert_eq!(digits.y_max, 2);
        assert_eq!(digits.elements, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(digits.get_element(&Point::new(2, 1)), Some(6));
    }

    #[test]
    fn try_map_errors() {
        let good = Grid::parse("12\n34");
        let parsed = good.try_map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| format!("not a digit: {c}"))
        });
        assert_eq!(parsed.unwrap().elements, vec![1, 2, 3, 4]);

        let bad = Grid::parse("1x\n34");
        let parsed = bad.try_map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| format!("not a digit: {c}"))
        });
        assert_eq!(parsed, Err("not a digit: x".to_string()));
    }
}
