//! # 2022 day 12 - Hill Climbing Algorithm
use crate::aoc_util::{grid::*, point::*};
use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    grid: Grid,
    start: Point,
    end: Point,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut grid = Grid::parse(input);
        let start = grid.find('S').unwrap();
        let end = grid.find('E').unwrap();
        grid.set_point(&start, 'a');
        grid.set_point(&end, 'z');
        Self { grid, start, end }
    }

    fn solve_part1(&self) -> usize {
        self.bfs(false)
    }

    fn solve_part2(&self) -> usize {
        self.bfs(true)
    }

    fn bfs(&self, backwards: bool) -> usize {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        if !backwards {
            queue.push_back((self.start, 0, Point::new(-1, -1)));
        } else {
            queue.push_back((self.end, 0, Point::new(-1, -1)));
        }
        while let Some((current, steps, prev)) = queue.pop_front() {
            if (!backwards && current == self.end)
                || (backwards && self.grid.get_element(&current).unwrap() == 'a')
            {
                return steps;
            }
            if seen.contains(&current) {
                continue;
            }
            seen.insert(current);
            let v = self.grid.get_element(&current).unwrap();
            for neighbor in NEIGHBORS_STRAIGHT.iter().map(|&n| n + current) {
                if neighbor == prev {
                    continue;
                }
                if let Some(e) = self.grid.get_element(&neighbor) {
                    if (!backwards && e as isize - v as isize <= 1)
                        || (backwards && v as isize - e as isize <= 1)
                    {
                        queue.push_back((neighbor, steps + 1, current));
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 31);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 29);
    }
}
