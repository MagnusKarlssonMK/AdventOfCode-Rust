use crate::aoc_util::point::{self, Point};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
enum Command {
    Up (usize),
    Down (usize),
    Forward (usize),
}

impl Command {
    fn from(s: &str) -> Self {
        let (_, right) = s.split_once(' ').unwrap();
        let val: usize = right.parse().unwrap();
        match s.chars().nth(0).unwrap() {
            'u' => Self::Up(val),
            'd' => Self::Down(val),
            'f' => Self::Forward(val),
            _ => unreachable!()
        }
    }
}

struct InputData {
    instructions: Vec<Command>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { instructions:
            input.lines()
                .map(|line| Command::from(&line))
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        let mut position = point::ORIGIN;
        for cmd in &self.instructions {
            match cmd {
                Command::Up(v) => position -= Point::new(0, *v as i32),
                Command::Down(v) => position += Point::new(0, *v as i32),
                Command::Forward(v) => position += Point::new(*v as i32, 0),
            };
        }
        (position.x * position.y).try_into().unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut position = point::ORIGIN;
        let mut aim = 0;
        for cmd in &self.instructions {
            match cmd {
                Command::Up(v) => aim -= *v as i32,
                Command::Down(v) => aim += *v as i32,
                Command::Forward(v) => position += Point::new(*v as i32, *v as i32 * aim),
            };
        }
        (position.x * position.y).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 150);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 900);
    }
}
