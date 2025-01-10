//! # 2017 day 7 - Recursive Circus
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<String> = s
            .split_whitespace()
            .map(|token| token.to_string())
            .collect();
        let mut leaflist: Vec<String> = Vec::new();
        if tokens.len() > 2 {
            for t in tokens.iter().skip(2) {
                leaflist.push(t.trim_end_matches(',').to_string());
            }
        }
        Ok(Self {
            weight: tokens[0]
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .parse()
                .unwrap(),
            leafs: leaflist,
        })
    }
}

struct InputData {
    programs: HashMap<String, Program>,
    root: String,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_and_prog_list: Vec<(String, Program)> = s
            .lines()
            .map(|line| {
                let (name, rest) = line.split_once(' ').unwrap();
                (name.to_string(), Program::from_str(rest).unwrap())
            })
            .collect();
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
            root: root.unwrap(),
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
                } else {
                    bad_weight = *weight;
                    bad_node = nodes.first().unwrap().to_string();
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

    const TEST_DATA: &str = "pbga (66)
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

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), String::from("tknk"));
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 60);
    }
}
