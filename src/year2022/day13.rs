//! # 2022 day 13 - Distress Signal
//!
//! Taking the opportunity to work with Box / Cons list and implementing the ordering trait.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{
    cmp::Ordering,
    error::Error,
    iter::Peekable,
    str::{Chars, FromStr},
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(PartialEq, Eq)]
enum Value {
    List(Box<Packet>),
    Integer(u32),
}

#[derive(PartialEq, Eq)]
enum Packet {
    Cons(Value, Box<Packet>),
    Empty,
}

impl FromStr for Packet {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_list(chars: &mut Peekable<Chars>) -> Result<Packet, AocError> {
            let value = match chars.next().ctx("unexpected end of packet")? {
                ']' => return Ok(Packet::Empty),
                '[' => Value::List(Box::new(parse_list(chars)?)),
                c => {
                    let mut n = c
                        .to_digit(10)
                        .ok_or_else(|| AocError::Invalid(format!("not a digit: {c}")))?;
                    while let Some(d) = chars.peek().and_then(|&c| c.to_digit(10)) {
                        chars.next();
                        n = n
                            .checked_mul(10)
                            .and_then(|n| n.checked_add(d))
                            .ok_or_else(|| {
                                AocError::Invalid("packet integer overflow".to_string())
                            })?;
                    }
                    Value::Integer(n)
                }
            };
            match chars.next().ctx("unexpected end of packet")? {
                ',' => Ok(Packet::Cons(value, Box::new(parse_list(chars)?))),
                ']' => Ok(Packet::Cons(value, Box::new(Packet::Empty))),
                c => Err(AocError::Invalid(format!("expected ',' or ']', got: {c}"))),
            }
        }

        let mut chars = s.chars().peekable();
        if chars.next() != Some('[') {
            return Err(AocError::Invalid("packet must start with '['".to_string()));
        }
        let packet = parse_list(&mut chars)?;
        if let Some(c) = chars.next() {
            return Err(AocError::Invalid(format!(
                "trailing character after packet: {c}"
            )));
        }
        Ok(packet)
    }
}

// `Eq` is derived structurally, while `Ord` compares by packet rules, so e.g. `[1]` and `[[1]]`
// compare `Equal` without being `==`. This is intentional: `sort` uses the packet ordering, while
// the divider lookup needs exact structural equality.
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Empty, Packet::Empty) => Ordering::Equal,
            (Packet::Empty, _) => Ordering::Less,
            (_, Packet::Empty) => Ordering::Greater,
            (Packet::Cons(v1, t1), Packet::Cons(v2, t2)) => match (v1, v2) {
                (Value::Integer(i1), Value::Integer(i2)) => match i1.cmp(i2) {
                    Ordering::Equal => t1.cmp(t2),
                    ordering => ordering,
                },
                (Value::Integer(i1), Value::List(list2)) => {
                    let converted_int = Packet::Cons(Value::Integer(*i1), Box::new(Packet::Empty));
                    match converted_int.cmp(list2) {
                        Ordering::Equal => t1.cmp(t2),
                        ordering => ordering,
                    }
                }
                (Value::List(list1), Value::Integer(i2)) => {
                    let converted_int = Packet::Cons(Value::Integer(*i2), Box::new(Packet::Empty));
                    match list1.cmp(&Box::new(converted_int)) {
                        Ordering::Equal => t1.cmp(t2),
                        ordering => ordering,
                    }
                }
                (Value::List(list1), Value::List(list2)) => match list1.cmp(list2) {
                    Ordering::Equal => t1.cmp(t2),
                    ordering => ordering,
                },
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct InputData {
    pairs: Vec<(Packet, Packet)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pairs(s: &str) -> Result<(Packet, Packet), AocError> {
            let (left, right) = s.split_once('\n').ctx("pair line break missing")?;
            Ok((left.parse()?, right.parse()?))
        }
        Ok(Self {
            pairs: s.split("\n\n").map(parse_pairs).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .filter(|(_, (left, right))| left < right)
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let divider_packet1: Packet = "[[2]]".parse().expect("divider packet must be correct");
        let divider_packet2: Packet = "[[6]]".parse().expect("divider packet must be correct");
        let mut packets = Vec::new();
        for (left, right) in self.pairs.iter() {
            packets.push(left);
            packets.push(right);
        }
        packets.push(&divider_packet1);
        packets.push(&divider_packet2);
        packets.sort();
        let pos1 = 1 + packets
            .iter()
            .position(|p| **p == divider_packet1)
            .expect("divider packet must be present");
        let pos2 = 1 + packets
            .iter()
            .position(|p| **p == divider_packet2)
            .expect("divider packet must be present");
        pos1 * pos2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 140);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "1",        // not a list
            "[1,2",     // truncated
            "[1,[2]",   // truncated nested list
            "[1]x",     // trailing garbage
            "[1,,2]",   // empty element
            "[[1][2]]", // missing comma
        ] {
            assert!(input.parse::<Packet>().is_err());
        }
    }

    #[test]
    fn parser_large_integer() {
        assert!("[300]".parse::<Packet>().is_ok());
        assert!("[4294967296]".parse::<Packet>().is_err());
    }
}
