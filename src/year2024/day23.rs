//! # 2024 day 23 - LAN Party
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2(),
    ))
}

struct InputData {
    connections: HashMap<String, HashSet<String>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
        s.lines().for_each(|line| {
            let (left, right) = line.split_once('-').unwrap();
            connections
                .entry(left.to_string())
                .and_modify(|v| {
                    v.insert(right.to_string());
                })
                .or_insert(HashSet::from([right.to_string()]));
            connections
                .entry(right.to_string())
                .and_modify(|v| {
                    v.insert(left.to_string());
                })
                .or_insert(HashSet::from([left.to_string()]));
        });
        Ok(Self { connections })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut seen = HashSet::new();
        self.connections
            .iter()
            .filter(|(k, _)| k.starts_with('t'))
            .for_each(|(k1, v)| {
                for k2 in v.iter() {
                    for k3 in self.connections.get(k2).unwrap().intersection(v) {
                        let mut t_set = [k1, k2, k3];
                        t_set.sort_unstable();
                        seen.insert(t_set);
                    }
                }
            });
        seen.len()
    }

    fn solve_part2(&self) -> String {
        let mut seen: HashSet<String> = HashSet::new();
        let mut group = Vec::new();
        let mut largest_group = Vec::new();

        for (k1, v1) in self.connections.iter() {
            if !seen.contains(k1) {
                group.clear();
                group.push(k1.clone());
                for k2 in v1 {
                    if group
                        .iter()
                        .all(|k3| self.connections.get(k2).unwrap().contains(k3))
                    {
                        seen.insert(k2.clone());
                        group.push(k2.clone());
                    }
                }
                if group.len() > largest_group.len() {
                    largest_group.clone_from(&group);
                }
            }
        }

        largest_group.sort_unstable();
        largest_group.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), "co,de,ka,ta");
    }
}
