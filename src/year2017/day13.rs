//! # 2017 day 13 - Packet Scanners
use crate::aoc_util::math;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct Scanner {
    depth: usize,
    range: usize,
    cycle: usize,
}

impl Scanner {
    fn parse(input: &str) -> Self {
        let (left, right) = input.split_once(": ").unwrap();
        let range = right.parse().unwrap();
        Self {
            depth: left.parse().unwrap(),
            range,
            cycle: 2 * (range - 1),
        }
    }

    #[inline]
    fn get_severity(&self) -> usize {
        self.depth * self.range
    }
}

struct InputData {
    scanners: Vec<Scanner>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            scanners: input.lines().map(Scanner::parse).collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        self.scanners
            .iter()
            .filter(|s| s.depth % s.cycle == 0)
            .map(|s| s.get_severity())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut current_lcm = 1;
        let mut delays = Vec::from([1]);
        for s in self.scanners.iter() {
            let new_lcm = math::lcm(current_lcm, s.cycle);
            let mut new_delays = Vec::new();
            for extra in (0..new_lcm).step_by(current_lcm) {
                delays
                    .iter()
                    .filter(|delay| (**delay + extra + s.depth) % s.cycle != 0)
                    .for_each(|delay| new_delays.push(*delay + extra));
            }
            current_lcm = new_lcm;
            delays = new_delays;
        }
        delays[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "0: 3
1: 2
4: 4
6: 4",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 24);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "0: 3
1: 2
4: 4
6: 4",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 10);
    }
}
