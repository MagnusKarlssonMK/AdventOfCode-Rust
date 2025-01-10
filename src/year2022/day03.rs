//! # 2022 day 3 - Rucksack Reorganization
use std::{collections::HashSet, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    rucksacks: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rucksacks: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            if c.is_uppercase() {
                                27 + (c as usize - 'A' as usize)
                            } else {
                                1 + (c as usize - 'a' as usize)
                            }
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.rucksacks
            .iter()
            .map(|rucksack| {
                let (left, right) = rucksack.split_at(rucksack.len() / 2);
                let a: HashSet<usize> = HashSet::from_iter(left.to_vec());
                let b: HashSet<usize> = HashSet::from_iter(right.to_vec());
                let common = a.intersection(&b).next().unwrap();
                *common
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.rucksacks
            .chunks(3)
            .map(|group| {
                // Can't figure out how to instersect more than two sets - brute force looping for now...
                let mut badge: usize = 0;
                for a in group[0].clone() {
                    if group[1].contains(&a) && group[2].contains(&a) {
                        badge = a;
                        break;
                    }
                }
                badge
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 157);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 70);
    }
}
