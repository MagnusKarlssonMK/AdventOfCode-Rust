// Type for handling 2-d grid points
use std::ops::{Add, AddAssign};
use std::hash::{Hash, Hasher};

pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const ORIGIN: Point = Point::new(0, 0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    #[inline]
    pub fn rotate_left(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    pub fn rotate_right(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    pub fn manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x as i32);
        state.write_i32(self.y as i32);
    }
}