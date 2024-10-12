use crate::aoc_util::point::*;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    steps: Vec<Point>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        fn translate(c: u8) -> Point {
            match c {
                b'>' => RIGHT,
                b'<' => LEFT,
                b'^' => UP,
                b'v' => DOWN,
                _ => ORIGIN,
            }
        }
        Self { steps: input.bytes().map(translate).collect() }
    }

    fn solve_part1(&self) -> usize {
        let mut santa = ORIGIN;
        let mut visited: HashSet<Point> = vec![santa].into_iter().collect();
        for step in self.steps.iter() {
            santa += *step;
            visited.insert(santa);
        }
        visited.len()
    }

    fn solve_part2(&self) -> usize {
        let mut santa = ORIGIN;
        let mut robosanta = ORIGIN;
        let mut visited: HashSet<Point> = vec![santa].into_iter().collect();
        for (count, step) in self.steps.iter().enumerate() {
            if count % 2 == 0 {
                santa += *step;
                visited.insert(santa);
            } else {
                robosanta += *step;
                visited.insert(robosanta);
            }
        }
        visited.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from(">");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("^>v<");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("^v^v^v^v^v");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("^v");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("^>v<");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from("^v^v^v^v^v");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 11);
    }
}
