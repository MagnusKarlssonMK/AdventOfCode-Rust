//! # 2025 day 11 - Reactor
//!
//! ## Part 1
//!
//! Simple recursive DFS to count number of paths from "you" to "out".
//!
//! ## Part 2
//!
//! Similar solution as in part 1, but now also passing along whether or not "fft" and "dac"
//! have been found in the recursion.
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    tree: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let tree = s
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(": ").unwrap();
                (left, right.split_whitespace().collect())
            })
            .collect();
        Ok(Self { tree })
    }
}

impl InputData<'_> {
    fn find_nbr_paths_straight(&self, key: &str, seen: &mut HashMap<String, u64>) -> u64 {
        if key == "out" {
            return 1;
        }
        let mut total = 0;
        if let Some(connections) = self.tree.get(key) {
            for connection in connections {
                let connection_string = connection.to_string();
                if let Some(v) = seen.get(&connection_string) {
                    total += v;
                } else {
                    let v = self.find_nbr_paths_straight(connection, seen);
                    seen.insert(connection_string, v);
                    total += v;
                }
            }
        }
        total
    }

    fn find_nbr_paths_fft_dac(
        &self,
        key: &str,
        seen: &mut HashMap<(String, bool, bool), u64>,
        fft: bool,
        dac: bool,
    ) -> u64 {
        if key == "out" {
            if fft && dac {
                return 1;
            } else {
                return 0;
            }
        }
        let fft = fft || key == "fft";
        let dac = dac || key == "dac";
        let mut total = 0;
        if let Some(connections) = self.tree.get(key) {
            for connection in connections {
                if let Some(v) = seen.get(&(connection.to_string(), fft, dac)) {
                    total += v;
                } else {
                    let v = self.find_nbr_paths_fft_dac(connection, seen, fft, dac);
                    seen.insert((connection.to_string(), fft, dac), v);
                    total += v;
                }
            }
        }
        total
    }

    fn solve_part1(&self) -> u64 {
        let mut seen = HashMap::new();
        self.find_nbr_paths_straight("you", &mut seen)
    }

    fn solve_part2(&self) -> u64 {
        let mut seen = HashMap::new();
        self.find_nbr_paths_fft_dac("svr", &mut seen, false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_DATA_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::try_from(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
