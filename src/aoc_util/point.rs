use std::error::Error;
// Type for handling 2-d points
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::str::FromStr;

pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const ORIGIN: Point = Point::new(0, 0);
pub const DIAG_R_D: Point = Point::new(1, 1);
pub const DIAG_L_D: Point = Point::new(-1, 1);
pub const DIAG_L_U: Point = Point::new(-1, -1);
pub const DIAG_R_U: Point = Point::new(1, -1);

pub const NEIGHBORS_STRAIGHT: [Point; 4] = [RIGHT, DOWN, LEFT, UP];

pub const NEIGHBORS_ALL: [Point; 8] = [
    RIGHT, DIAG_R_D, DOWN, DIAG_L_D, LEFT, DIAG_L_U, UP, DIAG_R_U,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn rotate_left(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    pub fn rotate_right(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    pub fn manhattan(&self, other: &Self) -> usize {
        (self.x - other.x).unsigned_abs() as usize + (self.y - other.y).unsigned_abs() as usize
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once([',', '-']) {
            Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            })
        } else {
            Err(format!("Can't parse to point: {}", s).into())
        }
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

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}
