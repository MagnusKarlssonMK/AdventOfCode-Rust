//! # 2023 day 19 - Aplenty
//!
//! Rating fields are converted to indices while parsing, so workflow rules can read and split
//! ranges without re-validating names.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

const X: usize = 0;
const M: usize = 1;
const A: usize = 2;
const S: usize = 3;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

fn field_index(name: &str) -> Option<usize> {
    match name {
        "x" => Some(X),
        "m" => Some(M),
        "a" => Some(A),
        "s" => Some(S),
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct Rating([usize; 4]);

impl FromStr for Rating {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let body = s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or_else(|| AocError::Invalid(format!("invalid rating: {s}")))?;
        let mut values = [0usize; 4];
        let mut seen = [false; 4];
        for part in body.split(',') {
            let (name, value) = part
                .split_once('=')
                .ok_or_else(|| AocError::Invalid(format!("invalid rating field: {part}")))?;
            let idx = field_index(name)
                .ok_or_else(|| AocError::Invalid(format!("unknown rating field: {name}")))?;
            if seen[idx] {
                return Err(AocError::Invalid(format!("duplicate rating field: {name}")));
            }
            seen[idx] = true;
            values[idx] = value.parse()?;
        }
        if !seen.iter().all(|&found| found) {
            return Err(AocError::Invalid(format!("incomplete rating: {s}")));
        }
        Ok(Self(values))
    }
}

impl Rating {
    fn get_value(&self, idx: usize) -> usize {
        self.0[idx]
    }

    fn set_value(&mut self, idx: usize, value: usize) {
        self.0[idx] = value;
    }

    fn get_combinations(&self, other: &Self) -> usize {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(high, low)| high.abs_diff(*low))
            .product()
    }
}

type PartRangeRating = (Rating, Rating);

/// Only the two comparisons that can appear in a rule.
#[derive(Clone, Copy)]
enum Comparison {
    Less,
    Greater,
}

struct Rule {
    condition: Option<(usize, Comparison, usize)>,
    next: String,
}

impl FromStr for Rule {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once(':') {
            let (name, comparison, val) = if let Some((name, val)) = left.split_once('<') {
                (name, Comparison::Less, val)
            } else if let Some((name, val)) = left.split_once('>') {
                (name, Comparison::Greater, val)
            } else {
                return Err(AocError::Invalid(format!("invalid rule: {s}")));
            };
            let idx = field_index(name)
                .ok_or_else(|| AocError::Invalid(format!("unknown rating field: {name}")))?;
            return Ok(Self {
                condition: Some((idx, comparison, val.parse()?)),
                next: right.to_string(),
            });
        }
        Ok(Self {
            condition: None,
            next: s.to_string(),
        })
    }
}

impl Rule {
    fn process_rule(&self, r: &Rating) -> Option<String> {
        match self.condition {
            Some((idx, Comparison::Less, val)) if r.get_value(idx) < val => Some(self.next.clone()),
            Some((idx, Comparison::Greater, val)) if r.get_value(idx) > val => {
                Some(self.next.clone())
            }
            Some(_) => None,
            None => Some(self.next.clone()),
        }
    }

    /// Splits the incoming range around the condition's threshold. The `None` name is the branch
    /// that fails the condition and continues through the remaining rules of this workflow.
    fn get_range_split(&self, ranges: &PartRangeRating) -> Vec<(Option<String>, PartRangeRating)> {
        let Some((idx, comparison, val)) = self.condition else {
            return vec![(Some(self.next.clone()), *ranges)];
        };
        if !(ranges.0.get_value(idx)..ranges.1.get_value(idx)).contains(&val) {
            return vec![(Some(self.next.clone()), *ranges)];
        }

        // The boundary is inclusive for `>` and exclusive for `<`, matching the half-open range.
        let split = match comparison {
            Comparison::Greater => val + 1,
            Comparison::Less => val,
        };
        let mut failing = *ranges;
        let mut passing = *ranges;
        failing.1.set_value(idx, split);
        passing.0.set_value(idx, split);
        match comparison {
            Comparison::Greater => vec![(None, failing), (Some(self.next.clone()), passing)],
            Comparison::Less => vec![(None, passing), (Some(self.next.clone()), failing)],
        }
    }
}

struct InputData {
    workflows: HashMap<String, Vec<Rule>>,
    ratings: Vec<Rating>,
}

impl FromStr for InputData {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (wf, rt) = s.split_once("\n\n").ctx("block separator missing")?;
        let mut workflows = HashMap::new();
        for line in wf.lines() {
            let (label, flow) = line
                .trim_end_matches('}')
                .split_once('{')
                .ok_or_else(|| AocError::Invalid(format!("bracket missing: {line}")))?;
            workflows.insert(
                label.to_string(),
                flow.split(',')
                    .map(|r| r.parse())
                    .collect::<Result<_, _>>()?,
            );
        }
        let ratings = rt
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { workflows, ratings })
    }
}

impl InputData {
    fn get_workflow(&self, name: &str) -> Result<&Vec<Rule>, AocError> {
        self.workflows
            .get(name)
            .ok_or_else(|| AocError::Invalid(format!("unknown workflow: {name}")))
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        const START: &str = "in";
        self.ratings
            .iter()
            .map(|r| {
                let mut flow = START.to_string();
                while flow != "A" && flow != "R" {
                    for rule in self.get_workflow(&flow)? {
                        if let Some(n) = rule.process_rule(r) {
                            flow = n;
                            break;
                        }
                    }
                }
                Ok(if flow == "A" { r.0.iter().sum() } else { 0 })
            })
            .sum()
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let startrange: PartRangeRating = (Rating([1; 4]), Rating([4001; 4]));
        let mut queue = VecDeque::from([("in".to_string(), startrange)]);
        let mut accepted = Vec::new();
        while let Some((workflow, mut ranges)) = queue.pop_front() {
            match workflow.as_str() {
                "A" => accepted.push(ranges),
                "R" => (),
                _ => {
                    for rule in self.get_workflow(&workflow)? {
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
        Ok(accepted
            .iter()
            .map(|(low, high)| high.get_combinations(low))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "px{a<2006:qkq,m>2090:A,rfg}
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
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 19114);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 167409079868000);
    }

    #[test]
    fn parser_invalid_rating() {
        for input in [
            "x=1,m=2,a=3",
            "{x=1,m=2,a=3}",
            "{x=1,m=2,a=3,q=4}",
            "{x=1,m=2,a=3,a=4}",
        ] {
            assert!(matches!(input.parse::<Rating>(), Err(AocError::Invalid(_))));
        }
        assert!(matches!(
            "{x=1,m=2,a=nope,s=4}".parse::<Rating>(),
            Err(AocError::ParseInt(_))
        ));
    }

    #[test]
    fn parser_invalid_rule() {
        for input in ["z<10:A", "x?10:A", "x:10"] {
            assert!(matches!(input.parse::<Rule>(), Err(AocError::Invalid(_))));
        }
    }

    #[test]
    fn unknown_workflow_is_an_error() {
        let solution_data: InputData = "in{nowhere}\n\n{x=1,m=1,a=1,s=1}".parse().unwrap();
        assert!(matches!(
            solution_data.solve_part1(),
            Err(AocError::Invalid(_))
        ));
        assert!(matches!(
            solution_data.solve_part2(),
            Err(AocError::Invalid(_))
        ));
    }
}
