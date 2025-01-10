//! # 2017 day 3 - Spiral Memory
use crate::aoc_util::point;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    puzzle_input: usize,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            puzzle_input: s.parse().unwrap(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut head = point::ORIGIN;
        let mut direction = point::DOWN;
        let mut value: usize = 1;
        let mut visited = HashSet::from([head]);
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
        let mut visited = HashMap::from([(head, value)]);
        while value < self.puzzle_input {
            let left_direction = direction.rotate_left();
            let left_point = head + left_direction;
            if visited.contains_key(&left_point) {
                head += direction;
            } else {
                direction = left_direction;
                head = left_point;
            }
            value = point::NEIGHBORS_ALL
                .iter()
                .filter_map(|d| visited.get(&(head + *d)))
                .sum();
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
        let testdata = "1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "12";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "23";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "1024";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 31);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "750";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 806);
    }
}
