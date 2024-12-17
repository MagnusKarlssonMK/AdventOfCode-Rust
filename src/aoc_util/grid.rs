use crate::aoc_util::point::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub x_max: usize,
    pub y_max: usize,
    pub elements: Vec<char>,
}

impl Grid {
    #[inline]
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

    pub fn new(x_max: usize, y_max: usize, val: char) -> Self {
        Self {
            x_max,
            y_max,
            elements: vec![val; x_max * y_max],
        }
    }

    #[inline]
    pub fn get_element(&self, p: &Point) -> Option<char> {
        if (0..self.x_max).contains(&(p.x as usize)) && (0..self.y_max).contains(&(p.y as usize)) {
            Some(self.elements[self.x_max * (p.y as usize) + (p.x as usize)])
        } else {
            None
        }
    }

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

    #[inline]
    pub fn find(&self, item: char) -> Option<Point> {
        self.elements
            .iter()
            .position(|&c| c == item)
            .map(|i| Point::new((i % self.x_max) as i32, (i / self.x_max) as i32))
    }

    #[inline]
    pub fn get_index(&self, p: &Point) -> usize {
        self.x_max * p.y as usize + p.x as usize
    }

    #[inline]
    pub fn get_point(&self, index: usize) -> Point {
        Point::new((index % self.x_max) as i32, (index / self.x_max) as i32)
    }

    pub fn print(&self) {
        for y in 0..self.y_max {
            for x in 0..self.x_max {
                print!("{}", self.elements[y * self.x_max + x]);
            }
            println!();
        }
        println!();
    }
}
