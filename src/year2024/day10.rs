use crate::aoc_util::point::*;
use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    map: Vec<Vec<usize>>,
    x_max: usize,
    y_max: usize,
    trailheads: Vec<Point>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut map = Vec::new();
        let mut trailheads = Vec:: new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in input.lines().enumerate() {
            y_max += 1;
            if y == 0 {
                x_max = line.len();
            }
            map.push(Vec::with_capacity(line.len()));
            for (x, c) in line.chars().enumerate() {
                let val = c.to_digit(10).unwrap();
                map[y].push(val as usize);
                if val == 0 {
                    trailheads.push(Point::new(x as i32, y as i32));
                }
            }
        }
        Self {
            map,
            x_max,
            y_max,
            trailheads
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut totalscore = 0;
        let mut totalrating = 0;
        for head in &self.trailheads {
            let mut peaks = HashSet::new();
            let mut seen = HashSet::new();
            let mut queue: VecDeque<(Point, Vec<Point>)> = VecDeque::new();
            queue.push_back((*head, vec![*head]));
            while let Some((current_point, current_seen)) = queue.pop_front() {
                if seen.contains(&(current_point, current_seen.clone())) {
                    continue;
                }
                seen.insert((current_point, current_seen.clone()));
                if self.map[current_point.y as usize][current_point.x as usize] == 9 {
                    peaks.insert(current_point);
                    totalrating += 1;
                } else {
                    for d in [LEFT, UP, RIGHT, DOWN] {
                        let new_x = current_point.x + d.x;
                        let new_y = current_point.y + d.y;
                        if (0..self.x_max as i32).contains(&new_x) && (0..self.y_max as i32).contains(&new_y) &&
                                self.map[new_y as usize][new_x as usize] == self.map[current_point.y as usize][current_point.x as usize] + 1 {
                            let mut new_seen: Vec<Point> = current_seen.clone();
                            new_seen.push(current_point);
                            queue.push_back((Point::new(new_x, new_y), new_seen));
                        }
                    }
                }
            }

            totalscore += peaks.len();
        }
        (totalscore, totalrating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts1_2_example_1() {
        let testdata = String::from(
"0123
1234
8765
9876");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1);
    }

    #[test]
    fn parts1_2_example_2() {
        let testdata = String::from(
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 36);
        assert_eq!(p2, 81);
    }
}
