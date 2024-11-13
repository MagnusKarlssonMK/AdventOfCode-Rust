use crate::aoc_util::point::*;
use std::str::FromStr;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

impl Point {
    fn is_diagonal(&self, other: &Point) -> bool {
        self.x != other.x && self.y != other.y
    }

    fn get_derivate(&self, other: &Point) -> Point {
        let dx = (other.x - self.x) / (other.x - self.x).abs().max((other.y - self.y).abs()).max(1);
        let dy = (other.y - self.y) / (other.x - self.x).abs().max((other.y - self.y).abs()).max(1);
        Point::new(dx, dy)
    }
}

struct InputData {
    lines: Vec<(Point, Point)>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        fn parse_line(line: &str) -> (Point, Point) {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            (Point::from_str(p1).unwrap(), Point::from_str(p2).unwrap())
        }
        Self {
            lines: input
                .lines()
                .map(parse_line)
                .collect()
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut grid = vec![vec![0; 1000]; 1000];
        // Note - this is significantly faster with an array for the grid, but gets too large and blows up the stack in test mode.
        let mut part1: usize = 0;
        let mut part2: usize = 0;
        for (p1, p2) in &self.lines {
            if !p1.is_diagonal(p2) {
                let dxdy = p1.get_derivate(p2);
                let mut p = *p1;
                loop {
                    if grid[p.x as usize][p.y as usize] == 1 {
                        part1 += 1;
                    }
                    grid[p.x as usize][p.y as usize] += 1;
                    if p == *p2 {
                        break;
                    }
                    p += dxdy;
                }
            }
        }

        // TBD - figure out a better way to do this and avoid duplicating almost the exact same code
        // Possibly try to separate the diagonal lines to a separate vector already at parsing
        part2 += part1;
        for (p1, p2) in &self.lines {
            if p1.is_diagonal(p2) {
                let dxdy = p1.get_derivate(p2);
                let mut p = *p1;
                loop {
                    if grid[p.x as usize][p.y as usize] == 1 {
                        part2 += 1;
                    }
                    grid[p.x as usize][p.y as usize] += 1;
                    if p == *p2 {
                        break;
                    }
                    p += dxdy;
                }
            }
        }
        (part1, part2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 5);
        assert_eq!(p2, 12);
    }
}
