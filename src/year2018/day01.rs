//! # 2018 day 1 - Chronal Calibration
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    changes: Vec<isize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            changes: s.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> isize {
        self.changes.iter().sum()
    }

    fn solve_part2(&self) -> isize {
        let mut i: usize = 0;
        let mut freq: isize = 0;
        let mut seen: HashSet<isize> = HashSet::new();
        seen.insert(freq);
        loop {
            freq += self.changes[i];
            if seen.contains(&freq) {
                break;
            }
            seen.insert(freq);
            i = (i + 1) % self.changes.len();
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "+1\n-2\n+3\n+1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "+1\n+1\n+1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "+1\n+1\n-2";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "-1\n-2\n-3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), -6);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "+1\n-2\n+3\n+1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "+1\n-1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "+3\n+3\n+4\n-2\n-4";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 10);
    }

    #[test]
    fn part2_example_4() {
        let testdata = "-6\n+3\n+8\n+5\n-6";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 5);
    }

    #[test]
    fn part2_example_5() {
        let testdata = "+7\n+7\n-2\n-7\n-4";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 14);
    }
}
