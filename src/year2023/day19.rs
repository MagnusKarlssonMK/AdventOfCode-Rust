//! # 2023 day 19 - Aplenty
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Clone, Copy)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Rating {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(['{', '}']).split(&[',', '=']).collect();
        Ok(Self {
            x: parts[1].parse().unwrap(),
            m: parts[3].parse().unwrap(),
            a: parts[5].parse().unwrap(),
            s: parts[7].parse().unwrap(),
        })
    }
}

impl Rating {
    fn get_value(&self, s: &str) -> Option<usize> {
        match s {
            "x" => Some(self.x),
            "m" => Some(self.m),
            "a" => Some(self.a),
            "s" => Some(self.s),
            _ => None,
        }
    }

    fn set_value(&mut self, s: &str, v: usize) {
        match s {
            "x" => self.x = v,
            "m" => self.m = v,
            "a" => self.a = v,
            "s" => self.s = v,
            _ => (),
        }
    }

    fn get_combinations(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x)
            * self.m.abs_diff(other.m)
            * self.a.abs_diff(other.a)
            * self.s.abs_diff(other.s)
    }
}

type PartRangeRating = (Rating, Rating);

struct Rule {
    condition: Option<(String, Ordering, usize)>,
    next: String,
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once(':') {
            if let Some((label, val)) = left.split_once('<') {
                return Ok(Self {
                    condition: Some((label.to_string(), Ordering::Less, val.parse().unwrap())),
                    next: right.to_string(),
                });
            }
            if let Some((label, val)) = left.split_once('>') {
                return Ok(Self {
                    condition: Some((label.to_string(), Ordering::Greater, val.parse().unwrap())),
                    next: right.to_string(),
                });
            }
        }
        Ok(Self {
            condition: None,
            next: s.to_string(),
        })
    }
}

impl Rule {
    fn process_rule(&self, r: &Rating) -> Option<String> {
        if let Some((name, order, val)) = &self.condition {
            match order {
                Ordering::Less => {
                    if r.get_value(name).unwrap() < *val {
                        Some(self.next.to_string())
                    } else {
                        None
                    }
                }
                Ordering::Greater => {
                    if r.get_value(name).unwrap() > *val {
                        Some(self.next.to_string())
                    } else {
                        None
                    }
                }
                Ordering::Equal => unreachable!(),
            }
        } else {
            Some(self.next.to_string())
        }
    }

    fn get_range_split(&self, ranges: &PartRangeRating) -> Vec<(Option<String>, PartRangeRating)> {
        if let Some((name, order, val)) = &self.condition {
            if (ranges.0.get_value(name).unwrap()..ranges.1.get_value(name).unwrap()).contains(val)
            {
                let mut new_low = *ranges;
                let mut new_high = *ranges;
                if *order == Ordering::Greater {
                    new_low.1.set_value(name, *val + 1);
                    new_high.0.set_value(name, *val + 1);
                    vec![(None, new_low), (Some(self.next.to_string()), new_high)]
                } else {
                    new_low.1.set_value(name, *val);
                    new_high.0.set_value(name, *val);
                    vec![(None, new_high), (Some(self.next.to_string()), new_low)]
                }
            } else {
                vec![(Some(self.next.to_string()), *ranges)]
            }
        } else {
            vec![(Some(self.next.to_string()), *ranges)]
        }
    }
}

struct InputData {
    workflows: HashMap<String, Vec<Rule>>,
    ratings: Vec<Rating>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (wf, rt) = s.split_once("\n\n").unwrap();
        let mut workflows = HashMap::new();
        for line in wf.lines() {
            let (label, flow) = line.trim_end_matches('}').split_once('{').unwrap();
            workflows.insert(
                label.to_string(),
                flow.split(',')
                    .map(|r| Rule::from_str(r).unwrap())
                    .collect(),
            );
        }
        let ratings = rt
            .lines()
            .map(|line| Rating::from_str(line).unwrap())
            .collect();
        Ok(Self { workflows, ratings })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        const START: &str = "in";
        self.ratings
            .iter()
            .map(|r| {
                let mut flow = START.to_string();
                while flow != "A" && flow != "R" {
                    for rule in self.workflows.get(&flow).unwrap() {
                        if let Some(n) = rule.process_rule(r) {
                            flow = n;
                            break;
                        }
                    }
                }
                if flow == "A" {
                    r.x + r.m + r.a + r.s
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let startrange: PartRangeRating = (
            Rating {
                x: 1,
                m: 1,
                a: 1,
                s: 1,
            },
            Rating {
                x: 4001,
                m: 4001,
                a: 4001,
                s: 4001,
            },
        );
        let mut queue = VecDeque::from([("in".to_string(), startrange)]);
        let mut accepted = Vec::new();
        while let Some((workflow, mut ranges)) = queue.pop_front() {
            match workflow.as_str() {
                "A" => {
                    accepted.push(ranges);
                }
                "R" => (),
                _ => {
                    for rule in self.workflows.get(&workflow).unwrap() {
                        let mut done = true;
                        for (new_name, new_ranges) in rule.get_range_split(&ranges) {
                            if let Some(n) = new_name {
                                queue.push_back((n, new_ranges));
                            } else {
                                done = false;
                                ranges = new_ranges;
                            }
                        }
                        if done {
                            break;
                        }
                    }
                }
            }
        }
        accepted
            .iter()
            .map(|(a_low, a_high)| a_high.get_combinations(a_low))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 19114);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 167409079868000);
    }
}
