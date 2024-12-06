use std::collections::HashSet;
use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    obstacles: HashSet<Point>,
    x_max: i32,
    y_max: i32,
    guard_start_pos: Point
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut start: Option<Point> = None;
        let mut obstacles = HashSet::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in input.lines().enumerate() {
            y_max += 1;
            if y == 0 {
                x_max = line.len() as i32;
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.insert(Point::new(x as i32, y as i32));
                } else if c == '^' {
                    start = Some(Point::new(x as i32, y as i32));
                }
            }
        }
        Self {
            obstacles,
            x_max,
            y_max,
            guard_start_pos: start.unwrap()
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut guard = self.guard_start_pos;
        let mut guard_dir = UP;
        while (0..self.x_max).contains(&guard.x) && (0..self.y_max).contains(&guard.y) {
            visited.insert(guard);
            let next_point = guard + guard_dir;
            if self.obstacles.contains(&next_point) {
                guard_dir = guard_dir.rotate_right();
            } else {
                guard = next_point;
            }
        }

        let mut p2 = 0;
        for extra_obstacle in &visited {
            if *extra_obstacle == self.guard_start_pos {
                continue;
            }
            guard = self.guard_start_pos;
            guard_dir = UP;
            let mut visited_directed: HashSet<(Point, Point)> = HashSet::new();
            while (0..self.x_max).contains(&guard.x) && (0..self.y_max).contains(&guard.y) {
                if visited_directed.contains(&(guard, guard_dir)) {
                    p2 += 1;
                    break;
                }
                visited_directed.insert((guard, guard_dir));
                let next_point = guard + guard_dir;
                if self.obstacles.contains(&next_point) || next_point == *extra_obstacle {
                    guard_dir = guard_dir.rotate_right();
                } else {
                    guard = next_point;
                }
            }
        }

        (visited.len(), p2)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 41);
        assert_eq!(p2, 6);
    }
}
