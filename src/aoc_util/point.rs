//! # Point utility
//!
//! Provides a struct and some useful functions for managing a 2D point in x-y plane.
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::str::FromStr;

use crate::aoc_util::error::AocError;

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

/// Struct for representing a point in 2D space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Rotates a point 90 degrees to the right. For example, x, y = (2, -3) becomes (3, -2).
    /// It is assumed that positive x goes to the right, and positive y goes down.
    #[inline]
    pub fn rotate_left(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    /// Rotates a point 90 degrees to the right. For example, x, y = (2, -3) becomes (3, -2).
    /// It is assumed that positive x goes to the right, and positive y goes down.
    #[inline]
    pub fn rotate_right(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    /// Calculates the manhattan distance between two points.
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
            Err(format!("Can't parse to point: {s}").into())
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

pub const ORIGIN_3D: Point3D = Point3D::new(0, 0, 0);

/// Struct for representing a point in 3D space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    /// Creates a new point.
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Calculates the 3D manhattan distance between two points.
    #[inline]
    pub fn manhattan(&self, other: &Point3D) -> usize {
        (self.x - other.x).unsigned_abs() as usize
            + (self.y - other.y).unsigned_abs() as usize
            + (self.z - other.z).unsigned_abs() as usize
    }
}

impl FromStr for Point3D {
    type Err = AocError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split([',']).collect::<Vec<_>>().as_slice() {
            [x, y, z] => Ok(Self {
                x: x.trim().parse()?,
                y: y.trim().parse()?,
                z: z.trim().parse()?,
            }),
            _ => Err(AocError::Invalid(format!("Can't parse to point3D: {s}"))),
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Point3D {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Point3D {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Point3D {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<i32> for Point3D {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Hash for Point3D {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
        state.write_i32(self.z);
    }
}
