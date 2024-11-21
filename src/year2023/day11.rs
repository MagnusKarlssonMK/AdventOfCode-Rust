use std::collections::HashSet;

use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve(2, 1_000_000);
    println!("Part 1: {}", p1);
    println!("Part 1: {}", p2);
}

struct InputData {
    galaxies: Vec<Point>,
    empty_x: HashSet<usize>,
    empty_y: HashSet<usize>
}

impl Point {
    fn get_x_ranges(&self, other: &Self) -> (usize, usize) {
        (self.x.min(other.x).try_into().unwrap(), self.x.max(other.x).try_into().unwrap())
    }

    fn get_y_ranges(&self, other: &Self) -> (usize, usize) {
        (self.y.min(other.y).try_into().unwrap(), self.y.max(other.y).try_into().unwrap())
    }
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut gal = Vec::new();
        let mut x_points = HashSet::new();
        let mut y_points = HashSet::new();
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        // My python solution didn't translate all that nicely into rust...
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    gal.push(Point::new(x as i32, y as i32));
                    x_points.insert(x);
                    y_points.insert(y);
                    x_min = x_min.min(x);
                    x_max = x_max.max(x);
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
            }
        }
        let mut empty_x = HashSet::new();
        let mut empty_y = HashSet::new();
        for x in x_min..x_max {
            if !x_points.contains(&x) {
                empty_x.insert(x);
            }
        }
        for y in y_min..y_max {
            if !y_points.contains(&y) {
                empty_y.insert(y);
            }
        }
        Self {
            galaxies: gal,
            empty_x,
            empty_y
        }
    }

    fn solve(&self, small_exp_rate: usize, large_exp_rate: usize) -> (usize, usize) {
        let mut total_steps: usize = 0;
        let mut total_empty_space: usize = 0;
        for (i, g1) in self.galaxies.iter().enumerate() {
            for g2 in self.galaxies.iter().skip(i + 1) {
                let x_range = g1.get_x_ranges(g2);
                let y_range = g1.get_y_ranges(g2);
                total_empty_space += self.empty_x.iter().filter(|x| (x_range.0+1..x_range.1).contains(x)).count() +
                    self.empty_y.iter().filter(|y| (y_range.0+1..y_range.1).contains(y)).count();
                total_steps += g1.manhattan(g2) as usize;
            }
        }
        (total_steps + total_empty_space * (small_exp_rate - 1),
        total_steps + total_empty_space * (large_exp_rate - 1))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts1_2_example_1() {
        let testdata = String::from("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve(2, 10);
        assert_eq!(p1, 374);
        assert_eq!(p2, 1030);
        let (p1, p2) = solution_data.solve(2, 100);
        assert_eq!(p1, 374);
        assert_eq!(p2, 8410);
    }
}
