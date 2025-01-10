//! # 2024 day 5 - Print Queue
use std::{cmp::Ordering, collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

enum UpdateResult {
    CorrectOrder(usize),
    CorrectedOrder(usize),
}

struct PageOrderingRules {
    rules: HashMap<(usize, usize), Ordering>,
}

impl FromStr for PageOrderingRules {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        for line in s.lines() {
            let (left, right) = line.split_once('|').unwrap();
            let left: usize = left.parse().unwrap();
            let right: usize = right.parse().unwrap();
            rules.insert((left, right), Ordering::Less);
        }
        Ok(Self { rules })
    }
}

impl PageOrderingRules {
    fn validate_update(&self, update: &[usize]) -> UpdateResult {
        let idx_mid = update.len() / 2;
        if update
            .is_sorted_by(|&left, &right| self.rules.get(&(left, right)) == Some(&Ordering::Less))
        {
            UpdateResult::CorrectOrder(update[idx_mid])
        } else {
            let mut correction = vec![0; update.len()];
            correction.copy_from_slice(update);
            correction.select_nth_unstable_by(idx_mid, |&left, &right| {
                match self.rules.get(&(left, right)) {
                    Some(r) => *r,
                    None => Ordering::Greater,
                }
            });
            UpdateResult::CorrectedOrder(correction[idx_mid])
        }
    }
}

struct InputData {
    rules: PageOrderingRules,
    updates: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, u) = s.split_once("\n\n").unwrap();
        Ok(Self {
            rules: PageOrderingRules::from_str(r).unwrap(),
            updates: u
                .lines()
                .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
                .collect(),
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        for update in &self.updates {
            match self.rules.validate_update(update) {
                UpdateResult::CorrectOrder(score) => p1 += score,
                UpdateResult::CorrectedOrder(score) => p2 += score,
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts1_2_example_1() {
        let testdata = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 143);
        assert_eq!(p2, 123);
    }
}
