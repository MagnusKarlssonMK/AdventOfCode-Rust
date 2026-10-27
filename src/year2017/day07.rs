//! # 2017 day 7 - Recursive Circus
use std::{collections::HashMap, error::Error, str::FromStr};

use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Program {
    weight: usize,
    leafs: Vec<String>,
}

impl FromStr for Program {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_weight(s: &str) -> Result<usize, AocError> {
            Ok(s.strip_prefix('(')
                .and_then(|w| w.strip_suffix(')'))
                .ok_or_else(|| AocError::Invalid(s.to_string()))?
                .parse::<usize>()?)
        }
        // s is either like "(66)" or "(72) -> ktlj, cntj, xhth"
        if let Some((weight_part, children_part)) = s.split_once(" -> ") {
            // Parse weight from weight_part
            let weight = extract_weight(weight_part)?;
            let leafs = children_part
                .split(',')
                .map(|c| c.trim().to_string())
                .collect();
            Ok(Program { weight, leafs })
        } else {
            // Just weight
            let weight = extract_weight(s)?;
            Ok(Program {
                weight,
                leafs: Vec::new(),
            })
        }
    }
}

struct InputData {
    programs: HashMap<String, Program>,
    root: String,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut name_and_prog_list: Vec<(String, Program)> = Vec::new();
        for line in s.lines() {
            if let Some((name, rest)) = line.split_once(' ') {
                name_and_prog_list.push((name.to_string(), rest.parse()?));
            } else {
                return Err(AocError::Invalid(line.to_string()));
            }
        }
        let mut parents: HashMap<String, String> = HashMap::new();
        for (p_name, prog) in name_and_prog_list.iter() {
            for leaf_name in prog.leafs.iter() {
                parents.insert(leaf_name.clone(), p_name.clone());
            }
        }
        let mut root = None;
        for (p_name, _) in name_and_prog_list.iter() {
            if !parents.contains_key(p_name) {
                root = Some(p_name.clone());
            }
        }
        Ok(Self {
            programs: HashMap::from_iter(name_and_prog_list),
            root: root.ok_or(AocError::Missing("no root"))?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> String {
        self.root.clone()
    }

    fn solve_part2(&self) -> usize {
        let (_, a) = self.get_weight_and_correction(&self.root);
        a as usize
    }

    fn get_weight_and_correction(&self, prog: &str) -> (usize, isize) {
        let mut leafweights: HashMap<usize, Vec<String>> = HashMap::new();
        let mut leafcorrections: Vec<isize> = Vec::new();
        if self.programs[prog].leafs.is_empty() {
            return (self.programs[prog].weight, 0);
        }

        for leaf in self.programs[prog].leafs.iter() {
            let (w, c) = self.get_weight_and_correction(leaf);
            leafweights
                .entry(w)
                .or_default()
                //.or_insert_with(Vec::new)
                .push(leaf.to_string());
            leafcorrections.push(c);
        }

        let correction: isize = leafcorrections.iter().sum();
        let leafcount: usize = leafweights.values().map(|v| v.len()).sum();
        if correction != 0 {
            // A node further down the tree has reported a correction, just propagate upwards
            return (
                self.programs[prog].weight + (leafweights.keys().sum::<usize>() * leafcount),
                correction,
            );
        } else if leafweights.len() > 1 {
            // These leafs are not balanced; figure out which one is bad and calculate the required correction
            let mut correct_weight: usize = 0;
            let mut bad_weight: usize = 0;
            let mut bad_node = String::from("");
            for (weight, nodes) in leafweights.iter() {
                if nodes.len() > 1 {
                    correct_weight = *weight;
                } else if let Some(n) = nodes.first() {
                    bad_weight = *weight;
                    bad_node = n.to_string();
                }
            }
            return (
                self.programs[prog].weight + (correct_weight * leafcount),
                correct_weight as isize
                    - (bad_weight as isize - self.programs[&bad_node].weight as isize),
            );
        } // else - all leafs have reported the same weight and are thus balanced
        (
            self.programs[prog].weight + (leafweights.keys().sum::<usize>() * leafcount),
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    const TEST_CUSTOM_1: &str = "A (10) -> B, D, E
B (20) -> C1, C2, C3
C1 (5)
C2 (5)
C3 (7)
D (35)
E (35)";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), String::from("tknk"));
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), String::from("A"));
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 60);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 5);
    }
}
