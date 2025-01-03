//! # 2022 day 13 - Distress Signal
//!
//! Taking the opportunity to work with Box / Cons list and implementing
//! the ordering trait.
use std::cmp::Ordering;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(PartialEq, Eq)]
enum Value {
    List(Box<Packet>),
    Integer(u8),
}

#[derive(PartialEq, Eq)]
enum Packet {
    Cons(Value, Box<Packet>),
    Empty,
}

impl Packet {
    fn new(input: &str) -> Self {
        fn parse_chars(c_input: &mut impl Iterator<Item = char>) -> Packet {
            let mut int_buffer: Option<u8> = None;
            while let Some(c) = c_input.next() {
                match c {
                    '[' => {
                        return Packet::Cons(
                            Value::List(Box::new(parse_chars(c_input))),
                            Box::new(parse_chars(c_input)),
                        )
                    }
                    ']' => {
                        if let Some(v) = int_buffer {
                            return Packet::Cons(Value::Integer(v), Box::new(Packet::Empty));
                        } else {
                            return Packet::Empty;
                        }
                    }
                    ',' => {
                        if let Some(v) = int_buffer {
                            return Packet::Cons(Value::Integer(v), Box::new(parse_chars(c_input)));
                        }
                    }
                    _ => {
                        let v = c.to_digit(10).unwrap() as u8;
                        if let Some(n) = int_buffer {
                            int_buffer = Some(10 * n + v);
                        } else {
                            int_buffer = Some(v)
                        }
                    }
                }
            }
            Packet::Empty
        }
        let mut chars = input.chars().skip(1);
        parse_chars(&mut chars)
    }
}

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

impl InputData {
    fn parse_input(input: &str) -> Self {
        let pairs = input
            .split("\n\n")
            .map(|pair| {
                let (left, right) = pair.split_once('\n').unwrap();
                (Packet::new(left), Packet::new(right))
            })
            .collect();
        Self { pairs }
    }

    fn solve_part1(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .filter(|(_, (left, right))| left < right)
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let divider_packet1 = Packet::new("[[2]]");
        let divider_packet2 = Packet::new("[[6]]");
        let mut packets = Vec::new();
        for (left, right) in self.pairs.iter() {
            packets.push(left);
            packets.push(right);
        }
        packets.push(&divider_packet1);
        packets.push(&divider_packet2);
        packets.sort();
        let pos1 = 1 + packets.iter().position(|p| **p == divider_packet1).unwrap();
        let pos2 = 1 + packets.iter().position(|p| **p == divider_packet2).unwrap();
        pos1 * pos2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "[1,1,3,1,1]
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
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 13);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "[1,1,3,1,1]
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
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 140);
    }
}
