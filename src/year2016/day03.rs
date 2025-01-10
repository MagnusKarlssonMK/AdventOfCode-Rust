//! # 2016 day 3 - Squares With Three Sides
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    sides: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sides: s.split_whitespace().map(|n| n.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.sides
            .chunks_exact(3)
            .filter(|s| validate_triangle(s))
            .count()
    }

    fn solve_part2(&self) -> usize {
        let column_wise: Vec<usize> = self
            .sides
            .iter()
            .step_by(3)
            .copied()
            .chain(
                self.sides
                    .iter()
                    .skip(1)
                    .step_by(3)
                    .copied()
                    .chain(self.sides.iter().skip(2).step_by(3).copied()),
            )
            .collect();
        column_wise
            .chunks_exact(3)
            .filter(|s| validate_triangle(s))
            .count()
    }
}

#[inline]
fn validate_triangle(sides: &[usize]) -> bool {
    sides[0] + sides[1] > sides[2]
        && sides[0] + sides[2] > sides[1]
        && sides[1] + sides[2] > sides[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "5 10 25";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 6);
    }
}
