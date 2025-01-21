//! # 2020 day 5 - Binary Boarding
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    boardingpasses: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boardingpasses: Vec<_> = s
            .lines()
            .map(|line| decode_seat_id(&line[0..7], &line[7..10]))
            .collect();
        boardingpasses.sort_unstable();
        Ok(Self { boardingpasses })
    }
}

fn decode_seat_id(rows: &str, cols: &str) -> usize {
    fn decode(s: &str, up: char, low: char, size: u8) -> usize {
        let mut lower = 0;
        let mut upper = size;
        for c in s.chars() {
            if c == up {
                upper -= (1 + upper - lower) / 2;
            } else if c == low {
                lower += (1 + upper - lower) / 2;
            }
        }
        lower as usize
    }
    8 * decode(rows, 'F', 'B', 127) + decode(cols, 'L', 'R', 7)
}

impl InputData {
    fn solve_part1(&self) -> usize {
        *self.boardingpasses.last().unwrap()
    }

    fn solve_part2(&self) -> usize {
        for w in self.boardingpasses.windows(2) {
            if w[1] == w[0] + 2 {
                return w[0] + 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str("BFFFBBFRRR").unwrap();
        assert_eq!(solution_data.solve_part1(), 567);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::from_str("FFFBBBFRRR").unwrap();
        assert_eq!(solution_data.solve_part1(), 119);
    }

    #[test]
    fn part1_example_3() {
        let solution_data = InputData::from_str("BBFFBBFRLL").unwrap();
        assert_eq!(solution_data.solve_part1(), 820);
    }
}
