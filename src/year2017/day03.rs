use crate::aoc_util::point;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    puzzle_input: usize,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            puzzle_input: input.parse().unwrap(),
        }
    }

    fn solve_part1(&self) -> usize {
        let mut head = point::ORIGIN;
        let mut direction = point::DOWN;
        let mut value: usize = 1;
        let mut visited: HashSet<point::Point> = vec![head].into_iter().collect();
        while value < self.puzzle_input {
            let left_direction = direction.rotate_left();
            let left_point = head + left_direction;
            if visited.contains(&left_point) {
                head += direction;
            } else {
                direction = left_direction;
                head = left_point;
            }
            visited.insert(head);
            value += 1;
        }
        head.manhattan(&point::ORIGIN)
    }

    fn solve_part2(&self) -> usize {
        let mut head = point::ORIGIN;
        let mut direction = point::DOWN;
        let mut value: usize = 1;
        let mut visited: HashMap<point::Point, usize> = vec![(head, value)].into_iter().collect();
        while value < self.puzzle_input {
            let left_direction = direction.rotate_left();
            let left_point = head + left_direction;
            if visited.contains_key(&left_point) {
                head += direction;
            } else {
                direction = left_direction;
                head = left_point;
            }
            value = 0;
            for neighbor_dir in point::NEIGHBORS_ALL {
                if let Some(neighborvalue) = visited.get(&(head + neighbor_dir)) {
                    value += neighborvalue;
                }
            }
            visited.insert(head, value);
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("12");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("23");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("1024");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 31);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("750");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 806);
    }
}
