//! # 2017 day 6 - Memory Reallocation
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve().to_string(),
        solution_data.solve().to_string(),
    ))
}

struct InputData {
    memorybanks: Vec<i32>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            memorybanks: s
                .split_whitespace()
                .map(|nbr| nbr.parse().unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn rebalance(&mut self) {
        let mut maxidx = self
            .memorybanks
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| -val)
            .unwrap()
            .0;
        // Note - need to use min-by and negate the key to get the first index, since
        // max-by returns the last index in case of duplicate max values
        let mut maxval = self.memorybanks[maxidx];
        let nbrbanks = self.memorybanks.len();
        self.memorybanks[maxidx] = 0;
        while maxval > 0 {
            maxidx = (maxidx + 1) % nbrbanks;
            self.memorybanks[maxidx] += 1;
            maxval -= 1;
        }
    }

    fn solve(&mut self) -> usize {
        let mut seen_configs = HashSet::new();
        while !seen_configs.contains(&self.memorybanks) {
            seen_configs.insert(self.memorybanks.clone());
            self.rebalance();
        }
        seen_configs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_2_example_1() {
        let testdata = "0 2 7 0";
        let mut solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve(), 5);
        assert_eq!(solution_data.solve(), 4);
    }
}
