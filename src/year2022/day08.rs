//! # 2022 day 8 - Treetop Tree House
//!
//! TBD - Figure out some way to compress duplicated code for all directions
//! with iterators.
use crate::aoc_util::{grid::*, point::*};
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    grid: Grid,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            grid: Grid::parse(input),
        }
    }

    fn solve_part1(&self) -> usize {
        let mut visible_trees = HashSet::new();
        let mut tallest;
        for y in 0..self.grid.y_max {
            tallest = -1;
            for x in 0..self.grid.x_max {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as i8;
                if v > tallest {
                    visible_trees.insert(p);
                    if v == 9 {
                        break;
                    }
                    tallest = v;
                }
            }
            tallest = -1;
            for x in (0..self.grid.x_max).rev() {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as i8;
                if v > tallest {
                    visible_trees.insert(p);
                    if v == 9 {
                        break;
                    }
                    tallest = v;
                }
            }
        }
        for x in 0..self.grid.x_max {
            tallest = -1;
            for y in 0..self.grid.y_max {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as i8;
                if v > tallest {
                    visible_trees.insert(p);
                    if v == 9 {
                        break;
                    }
                    tallest = v;
                }
            }
            tallest = -1;
            for y in (0..self.grid.y_max).rev() {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as i8;
                if v > tallest {
                    visible_trees.insert(p);
                    if v == 9 {
                        break;
                    }
                    tallest = v;
                }
            }
        }
        visible_trees.len()
    }

    fn solve_part2(&self) -> usize {
        let mut scorelist = vec![0; 10];
        let mut gridscore = vec![1; self.grid.x_max * self.grid.y_max];
        for y in 0..self.grid.y_max {
            // Right
            for s in &mut scorelist {
                *s = 0;
            }
            for x in 0..self.grid.x_max {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as usize;
                let idx = self.grid.get_index(&p);
                gridscore[idx] *= scorelist[v];
                for (i, s) in &mut scorelist.iter_mut().enumerate() {
                    if i > v {
                        *s += 1;
                    } else {
                        *s = 1;
                    };
                }
            }
            // Left
            for s in &mut scorelist {
                *s = 0;
            }
            for x in (0..self.grid.x_max).rev() {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as usize;
                let idx = self.grid.get_index(&p);
                gridscore[idx] *= scorelist[v];
                for (i, s) in &mut scorelist.iter_mut().enumerate() {
                    if i > v {
                        *s += 1;
                    } else {
                        *s = 1;
                    };
                }
            }
        }
        let mut largest = 0;
        for x in 0..self.grid.x_max {
            // Down
            for s in &mut scorelist {
                *s = 0;
            }
            for y in 0..self.grid.y_max {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as usize;
                let idx = self.grid.get_index(&p);
                gridscore[idx] *= scorelist[v];
                for (i, s) in &mut scorelist.iter_mut().enumerate() {
                    if i > v {
                        *s += 1;
                    } else {
                        *s = 1;
                    };
                }
            }
            // Up
            for s in &mut scorelist {
                *s = 0;
            }
            for y in (0..self.grid.y_max).rev() {
                let p = Point::new(x as i32, y as i32);
                let v = self.grid.get_uint_element(&p).unwrap() as usize;
                let idx = self.grid.get_index(&p);
                gridscore[idx] *= scorelist[v];
                largest = largest.max(gridscore[idx]);
                for (i, s) in &mut scorelist.iter_mut().enumerate() {
                    if i > v {
                        *s += 1;
                    } else {
                        *s = 1;
                    };
                }
            }
        }
        largest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "30373
25512
65332
33549
35390",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 21);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "30373
25512
65332
33549
35390",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 8);
    }
}
