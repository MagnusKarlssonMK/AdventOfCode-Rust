//! # 2024 day 5 - Print Queue
use std::{cmp::Ordering, collections::HashMap};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

enum UpdateResult {
    CorrectOrder(usize),
    CorrectedOrder(usize),
}

struct PageOrderingRules {
    rules: HashMap<(usize, usize), Ordering>,
}

impl PageOrderingRules {
    fn parse_str(input: &str) -> Self {
        let mut rules = HashMap::new();
        for line in input.lines() {
            let (left, right) = line.split_once('|').unwrap();
            let left: usize = left.parse().unwrap();
            let right: usize = right.parse().unwrap();
            rules.insert((left, right), Ordering::Less);
        }
        Self { rules }
    }

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

impl InputData {
    fn parse_input(input: &str) -> Self {
        let (r, u) = input.split_once("\n\n").unwrap();
        Self {
            rules: PageOrderingRules::parse_str(r),
            updates: u
                .lines()
                .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
                .collect(),
        }
    }

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
        let testdata = String::from(
            "47|53
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
97,13,75,29,47",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 143);
        assert_eq!(p2, 123);
    }
}
