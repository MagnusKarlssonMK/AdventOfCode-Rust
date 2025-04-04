//! # 2015 day 7 - Some Assembly Required
//!
//! Stores the input in a dictionary and then calculates the result by recursion
//! including a cache.
//! Can possibly be improved by removing duplicated code for extracting the node
//! values in get_values(). Maybe convert the strings to usize values to use as
//! keys instead to avoid having to deal with strings?
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

enum Node {
    Wire(String),
    Number(u16),
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<u16>() {
            Ok(Self::Number(v))
        } else {
            Ok(Self::Wire(s.to_string()))
        }
    }
}

enum Gate {
    Plain(Node),
    Not(Node),
    And(Node, Node),
    Or(Node, Node),
    Lshift(Node, u8),
    Rshift(Node, u8),
}

impl FromStr for Gate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let first = tokens.next().unwrap();
        if let Some(second) = tokens.next() {
            if let Some(third) = tokens.next() {
                match second {
                    "AND" => Ok(Self::And(
                        Node::from_str(first).unwrap(),
                        Node::from_str(third).unwrap(),
                    )),
                    "OR" => Ok(Self::Or(
                        Node::from_str(first).unwrap(),
                        Node::from_str(third).unwrap(),
                    )),
                    "LSHIFT" => Ok(Self::Lshift(
                        Node::from_str(first).unwrap(),
                        third.parse().unwrap(),
                    )),
                    "RSHIFT" => Ok(Self::Rshift(
                        Node::from_str(first).unwrap(),
                        third.parse().unwrap(),
                    )),
                    _ => unreachable!(),
                }
            } else {
                Ok(Self::Not(Node::from_str(second).unwrap()))
            }
        } else {
            Ok(Self::Plain(Node::from_str(first).unwrap()))
        }
    }
}

struct InputData {
    circuit: HashMap<String, Gate>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            circuit: s
                .lines()
                .map(|line| {
                    let (left, right) = line.split_once(" -> ").unwrap();
                    (right.to_string(), Gate::from_str(left).unwrap())
                })
                .collect(),
        })
    }
}

impl InputData {
    fn get_value(&self, v: &str, wires: &mut HashMap<String, u16>) -> u16 {
        if let Some(w) = wires.get(v) {
            *w
        } else {
            let result = match self.circuit.get(v).unwrap() {
                Gate::Plain(a) => match a {
                    Node::Number(n) => *n,
                    Node::Wire(w) => self.get_value(w, wires),
                },
                Gate::And(a, b) => {
                    let left = match a {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    let right = match b {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    left & right
                }
                Gate::Or(a, b) => {
                    let left = match a {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    let right = match b {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    left | right
                }
                Gate::Not(a) => {
                    let left = match a {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    !left
                }
                Gate::Lshift(a, b) => {
                    let left = match a {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    left << b
                }
                Gate::Rshift(a, b) => {
                    let left = match a {
                        Node::Number(n) => *n,
                        Node::Wire(w) => self.get_value(w, wires),
                    };
                    left >> b
                }
            };
            wires.insert(v.to_string(), result);
            result
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut wires = HashMap::new();
        let p1 = self.get_value("a", &mut wires);
        wires.clear();
        wires.insert("b".to_string(), p1);
        let p2 = self.get_value("a", &mut wires);
        (p1 as usize, p2 as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: changed d->a in the example input to get it to match the actual target
    const TEST_DATA: &str = "123 -> x
456 -> y
x AND y -> a
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 72);
    }
}
