//! # 20xx day xx - 
use std::error::Error;

pub fn solve(input: &str) {
    let solution_data = InputData::from_str(input).unwrap();
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    data: Vec<String>
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { data: s.lines().map(|line| line.to_string()).collect() })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        for line in &self.data {
            println!("{}", line);
        }
        1
    }

    fn solve_part2(&self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "2";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
