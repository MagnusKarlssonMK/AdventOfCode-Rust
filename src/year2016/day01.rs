use crate::aoc_util::point::*;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

enum Rotation {
    Left,
    Right
}

impl Rotation {
    fn from(s: &str) -> Self {
        if s == "R" {
            Self::Right
        } else {
            Self::Left
        }
    }
}

struct InputData {
    instructions: Vec<(Rotation, i32)>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { instructions: input.split(", ")
            .map(|word| {
                let (left, right) = word.split_at(1);
                (Rotation::from(left), right.parse().unwrap())
            })
            .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        let mut pos = ORIGIN;
        let mut direction = UP;
        for (r, i) in &self.instructions {
            direction = match r {
                Rotation::Left => direction.rotate_left(),
                Rotation::Right => direction.rotate_right()
            };
            pos += direction * *i;
        };
        pos.manhattan(&ORIGIN)
    }

    fn solve_part2(&self) -> usize {
        let mut pos = ORIGIN;
        let mut direction = UP;
        let mut seen: HashSet<Point> = HashSet::new();
        seen.insert(pos);
        for (r, i) in &self.instructions {
            direction = match r {
                Rotation::Left => direction.rotate_left(),
                Rotation::Right => direction.rotate_right()
            };
            for _ in 0..*i {
                pos += direction;
                if seen.contains(&pos) {
                    return pos.manhattan(&ORIGIN);
                }
                seen.insert(pos);
            }
        };
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("R2, L3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("R2, R2, R2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("R5, L5, R5, R3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("R8, R4, R4, R8");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
