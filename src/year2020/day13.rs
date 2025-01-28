//! # 2020 day 13 - Shuttle Search
//!
//! Chinese remainder theorem solution.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    estimate: usize,
    buslist: Vec<(usize, usize)>,  // busid, offset-idx
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let estimate = lines.next().unwrap().parse().unwrap();
        let buslist = lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter(|(_, v)| *v != "x")
            .map(|(i, v)| (v.parse().unwrap(), i))
            .collect();
        Ok(Self { estimate, buslist })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let best_bus = self
            .buslist
            .iter()
            .map(|(id, _)| (id - (self.estimate % id), *id))
            .min()
            .unwrap();
        best_bus.0 * best_bus.1
    }

    fn solve_part2(&self) -> usize {
        let mut coefficient = self.buslist[0].0;
        let mut time = self.buslist[0].1;
        for (busid, idx) in &self.buslist[1..] {
            let remainder = busid - idx % busid;
            while time % busid != remainder {
                time += coefficient;
            }
            coefficient *= busid;
        }
        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 295);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 1068781);
    }
}
