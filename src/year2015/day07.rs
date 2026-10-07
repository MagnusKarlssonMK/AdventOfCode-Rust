//! # 2015 day 7 - Some Assembly Required
//!
//! Stores the input in a dictionary and then calculates the result by recursion
//! including a cache.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug)]
enum Node {
    Wire(String),
    Number(u16),
}

impl FromStr for Node {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<u16>() {
            Ok(Self::Number(v))
        } else {
            Ok(Self::Wire(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Gate {
    Plain(Node),
    Not(Node),
    And(Node, Node),
    Or(Node, Node),
    Lshift(Node, Node),
    Rshift(Node, Node),
}

impl FromStr for Gate {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let first = tokens.next().ctx("gate")?;
        if let Some(second) = tokens.next() {
            if let Some(third) = tokens.next() {
                match second {
                    "AND" => Ok(Self::And(Node::from_str(first)?, Node::from_str(third)?)),
                    "OR" => Ok(Self::Or(Node::from_str(first)?, Node::from_str(third)?)),
                    "LSHIFT" => Ok(Self::Lshift(Node::from_str(first)?, Node::from_str(third)?)),
                    "RSHIFT" => Ok(Self::Rshift(Node::from_str(first)?, Node::from_str(third)?)),
                    _ => Err(AocError::Invalid(s.to_string())),
                }
            } else {
                Ok(Self::Not(Node::from_str(second)?))
            }
        } else {
            Ok(Self::Plain(Node::from_str(first)?))
        }
    }
}

#[derive(Debug)]
struct InputData {
    circuit: HashMap<String, Gate>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            circuit: s
                .lines()
                .map(|line| {
                    let (left, right) = line.split_once(" -> ").ctx("-> separator")?;
                    Ok((right.to_string(), Gate::from_str(left)?))
                })
                .collect::<Result<HashMap<_, _>, AocError>>()?,
        })
    }
}

impl InputData {
    fn get_value(&self, v: &str, wires: &mut HashMap<String, u16>) -> Result<u16, AocError> {
        if let Some(w) = wires.get(v) {
            Ok(*w)
        } else {
            let resolve = |node: &Node, wires: &mut HashMap<String, u16>| match node {
                Node::Number(n) => Ok(*n),
                Node::Wire(w) => self.get_value(w, wires),
            };
            let result = match self.circuit.get(v).ctx("undefined wire")? {
                Gate::Plain(a) => resolve(a, wires)?,
                Gate::And(a, b) => resolve(a, wires)? & resolve(b, wires)?,
                Gate::Or(a, b) => resolve(a, wires)? | resolve(b, wires)?,
                Gate::Not(a) => !resolve(a, wires)?,
                Gate::Lshift(a, b) => resolve(a, wires)? << resolve(b, wires)?,
                Gate::Rshift(a, b) => resolve(a, wires)? >> resolve(b, wires)?,
            };
            wires.insert(v.to_string(), result);
            Ok(result)
        }
    }

    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut wires = HashMap::new();
        let p1 = self.get_value("a", &mut wires)?;
        wires.clear();
        wires.insert("b".to_string(), p1);
        let p2 = self.get_value("a", &mut wires)?;
        Ok((p1 as usize, p2 as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CUSTOM_1: &str = "15 -> x
22 -> y
1 -> b
x AND y -> c
c OR b -> d
d RSHIFT 1 -> e
e LSHIFT 1 -> f
NOT f -> a";

    const TEST_CUSTOM_2: &str = "300 -> x
x LSHIFT 8 -> a";

    const TEST_CUSTOM_3: &str = "1 -> b
3 -> d
7 -> x
1 AND x -> c
d LSHIFT b -> a";

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 65529);
        assert_eq!(p2, 1)
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 11264);
        assert_eq!(p2, 11264)
    }

    #[test]
    fn part1_2_custom_3() {
        let solution_data = InputData::from_str(TEST_CUSTOM_3).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 6);
        assert_eq!(p2, 192)
    }

    #[test]
    fn parse_invalid_operator() {
        let err = InputData::from_str("x FOO y -> z").unwrap_err();
        assert_eq!(err, AocError::Invalid("x FOO y".to_string()))
    }

    #[test]
    fn parse_invalid_separator() {
        let err = InputData::from_str("x RSHIFT y - z").unwrap_err();
        assert_eq!(err, AocError::Missing("-> separator"))
    }

    #[test]
    fn undefined_wire_ref() {
        let solution_data = InputData::from_str("b -> a").unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Missing("undefined wire"));
    }
}
