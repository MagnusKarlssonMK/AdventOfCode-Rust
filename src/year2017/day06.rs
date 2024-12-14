use std::collections::HashSet;

pub fn solve(input: &str) {
    let mut solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve());
    println!("Part 2: {}", solution_data.solve());
}

struct InputData {
    memorybanks: Vec<i32>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            memorybanks: input
                .split_whitespace()
                .map(|nbr| nbr.parse().unwrap())
                .collect(),
        }
    }

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
        let testdata = String::from("0 2 7 0");
        let mut solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve(), 5);
        assert_eq!(solution_data.solve(), 4);
    }
}
