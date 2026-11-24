//! # 2017 day 3 - Spiral Memory
use crate::aoc_util::{error::AocError, point};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    puzzle_input: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            puzzle_input: s.parse()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut head = point::ORIGIN;
        let mut direction = point::DOWN;
        let mut value = 1;
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
    fn part1_official_1() {
        let solution_data: InputData = "1".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = "12".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = "23".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_official_4() {
        let solution_data: InputData = "1024".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 31);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = "750".parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 806);
    }
}
