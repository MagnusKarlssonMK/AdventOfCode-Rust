//! # 2022 day 11 - Monkey in the Middle
use crate::aoc_util::math;
use std::collections::VecDeque;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn new(input: &str) -> Self {
        let (_, right) = input.split_once(" = ").unwrap();
        let right: Vec<_> = right.split_whitespace().take(3).collect();
        match right[1] {
            "+" => Self::Add(right[2].parse().unwrap()),
            "*" => {
                if right[2] == "old" {
                    Self::Square
                } else {
                    Self::Mul(right[2].parse().unwrap())
                }
            }
            _ => unreachable!(),
        }
    }

    fn calculate_new(&self, old: usize) -> usize {
        match self {
            Self::Add(n) => old + n,
            Self::Mul(n) => old * n,
            Self::Square => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    div_test: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: usize,
}

impl Monkey {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next();
        let (_, nbrs) = lines.next().unwrap().split_once(": ").unwrap();
        let start_items = nbrs.split(", ").map(|n| n.parse().unwrap()).collect();
        let operation = Operation::new(lines.next().unwrap());
        let div_test = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let true_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            items: start_items,
            operation,
            div_test,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        }
    }

    fn inspect(&mut self, apply_relief: bool, r: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        while !self.items.is_empty() {
            let mut current_item = self.items.pop_front().unwrap();
            self.inspect_count += 1;
            current_item = self.operation.calculate_new(current_item);
            if apply_relief {
                current_item /= 3;
            } else {
                current_item %= r;
            }
            if current_item % self.div_test == 0 {
                result.push((self.true_monkey, current_item));
            } else {
                result.push((self.false_monkey, current_item));
            }
        }
        result
    }
}

struct InputData {
    monkeys: Vec<Monkey>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            monkeys: input.split("\n\n").map(Monkey::new).collect(),
        }
    }

    fn play(&self, rounds: usize, apply_relief: bool) -> usize {
        let mut monkeys = self.monkeys.clone();
        let r = if apply_relief {
            1
        } else {
            math::vec_lcm(&(monkeys.iter().map(|m| m.div_test).collect::<Vec<usize>>()))
        };
        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                for (destination, item) in &monkeys[i].inspect(apply_relief, r) {
                    monkeys[*destination].items.push_back(*item);
                }
            }
        }
        let mut inspectcounts: Vec<usize> = monkeys.iter().map(|m| m.inspect_count).collect();
        inspectcounts.sort_unstable();
        inspectcounts.reverse();
        inspectcounts[0] * inspectcounts[1]
    }

    fn solve_part1(&self) -> usize {
        self.play(20, true)
    }

    fn solve_part2(&self) -> usize {
        self.play(10_000, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 10605);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2713310158);
    }
}
