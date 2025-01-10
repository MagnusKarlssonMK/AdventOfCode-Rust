//! # 2015 day 2 - I Was Told There Would Be No Math
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    gifts: Vec<(usize, usize, usize)>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            gifts: s
                .lines()
                .map(|line| {
                    let mut sides = line
                        .split('x')
                        .map(|digit| digit.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    sides.sort_unstable();
                    if sides.len() != 3 {
                        (0, 0, 0)
                    } else {
                        (sides[0], sides[1], sides[2])
                    }
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.gifts
            .iter()
            .map(|(l, w, h)| 3 * (l * w) + 2 * h * (w + l))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.gifts
            .iter()
            .map(|(l, w, h)| 2 * (l + w) + (l * w * h))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = "2x3x4";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 58);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "1x1x10";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 43);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "2x3x4";
        let solution_data = InputData::from_str(&testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 34);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "1x1x10";
        let solution_data = InputData::from_str(&testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 14);
    }
}
