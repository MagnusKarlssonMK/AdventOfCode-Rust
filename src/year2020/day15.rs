//! # 2020 day 15 - Rambunctious Recitation
//!
//! Basically the Van Eck sequence. Seems to require brute-forcing, so part 2
//! is really slow and uses a lot of RAM to keep track of the counters for each
//! number. Use u32 rather than usize to at least keep it somewhat under control.
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    startlist: Vec<u32>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            startlist: s.split(',').map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn play(&self, rounds: usize) -> usize {
        let mut nbrs: HashMap<u32, u32> = HashMap::from_iter(
            self.startlist
                .iter()
                .enumerate()
                .rev()
                .skip(1)
                .map(|(i, &n)| (n, i as u32)),
        );
        let mut lastspoken = *self.startlist.last().unwrap();
        for turn in self.startlist.len()..rounds {
            let speak = if let Some(n) = nbrs.get(&lastspoken) {
                turn as u32 - n - 1
            } else {
                0
            };
            nbrs.insert(lastspoken, turn as u32 - 1);
            lastspoken = speak;
        }
        lastspoken as usize
    }

    fn solve_part1(&self) -> usize {
        self.play(2020)
    }

    fn solve_part2(&self) -> usize {
        self.play(30_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "0,3,6";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 436);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "1,3,2";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "2,1,3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "1,2,3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 27);
    }

    #[test]
    fn part1_example_5() {
        let testdata = "2,3,1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 78);
    }

    #[test]
    fn part1_example_6() {
        let testdata = "3,2,1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 438);
    }

    #[test]
    fn part1_example_7() {
        let testdata = "3,1,2";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1836);
    }

    // Skip part 2 test, since it's really slow and anyway just a lot more iterations
    // of the same function.
}
