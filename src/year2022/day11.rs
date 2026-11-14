//! # 2022 day 11 - Monkey in the Middle
//!
//! Each monkey block parses into starting items, a linear operation, and a divisibility test with
//! two destinations. Both parts replay the same rounds: every monkey inspects all of its items,
//! applies the operation, and throws each item to one of two monkeys. Part 1 divides the worry
//! level by 3 after every inspection, while part 2 keeps the numbers bounded by reducing them
//! modulo the least common multiple of all divisibility tests, which preserves every test result.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    math,
};
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl FromStr for Operation {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Operation:", "new", "=", "old", "*", "old"] => Ok(Operation::Square),
            ["Operation:", "new", "=", "old", "*", v] => Ok(Operation::Mul(v.parse()?)),
            ["Operation:", "new", "=", "old", "+", v] => Ok(Operation::Add(v.parse()?)),
            _ => Err(AocError::Invalid(format!("invalid operation: {s}"))),
        }
    }
}

impl Operation {
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

impl FromStr for Monkey {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn last_number(line: Option<&str>, what: &str) -> Result<usize, AocError> {
            Ok(line
                .ctx(format!("{what} line missing"))?
                .split_whitespace()
                .last()
                .ctx(format!("{what} line empty"))?
                .parse()?)
        }

        let mut lines = s.lines();
        lines.next();
        let (_, nbrs) = lines
            .next()
            .ctx("items line missing")?
            .split_once(": ")
            .ctx("items ': ' separator")?;
        let start_items = nbrs
            .split(", ")
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        let operation = lines.next().ctx("operation line missing")?.parse()?;

        let div_test = last_number(lines.next(), "div test")?;
        let true_monkey = last_number(lines.next(), "true monkey")?;
        let false_monkey = last_number(lines.next(), "false monkey")?;

        if div_test == 0 {
            return Err(AocError::Invalid("divisor must be non-zero".to_string()));
        }
        Ok(Self {
            items: start_items,
            operation,
            div_test,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        })
    }
}

impl Monkey {
    fn inspect(&mut self, apply_relief: bool, r: usize, throws: &mut Vec<(usize, usize)>) {
        throws.clear();
        // Note: throws stored in a reused pre-allocated vec instead of returning a freshly
        // allocated one every time, since this function is called in a hot loop.
        while let Some(item) = self.items.pop_front() {
            self.inspect_count += 1;
            let mut current_item = self.operation.calculate_new(item);
            if apply_relief {
                current_item /= 3;
            } else {
                current_item %= r;
            }
            if current_item.is_multiple_of(self.div_test) {
                throws.push((self.true_monkey, current_item));
            } else {
                throws.push((self.false_monkey, current_item));
            }
        }
    }
}

struct InputData {
    monkeys: Vec<Monkey>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys: Vec<_> = s
            .split("\n\n")
            .map(|m| m.parse())
            .collect::<Result<_, _>>()?;
        if monkeys.len() < 2 {
            Err(AocError::Invalid(
                "need at least 2 monkeys to play".to_string(),
            ))
        } else {
            Ok(Self { monkeys })
        }
    }
}

impl InputData {
    fn play(&self, rounds: usize, apply_relief: bool) -> Result<usize, AocError> {
        let mut monkeys = self.monkeys.clone();
        let r = if apply_relief {
            1
        } else {
            monkeys
                .iter()
                .map(|m| m.div_test)
                .reduce(math::lcm)
                .ok_or_else(|| AocError::Invalid("monkeys can not be empty for LCM".to_string()))?
        };
        let mut throws = Vec::new();
        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                monkeys[i].inspect(apply_relief, r, &mut throws);
                for &(destination, item) in &throws {
                    monkeys
                        .get_mut(destination)
                        .ok_or_else(|| {
                            AocError::Invalid(format!(
                                "invalid destination monkey: {}",
                                destination
                            ))
                        })?
                        .items
                        .push_back(item);
                }
            }
        }
        let mut inspectcounts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
        inspectcounts.sort_unstable();
        inspectcounts.reverse();
        Ok(inspectcounts[0] * inspectcounts[1])
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        self.play(20, true)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        self.play(10_000, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Monkey 0:
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
    If false: throw to monkey 1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 10605);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 2713310158);
    }

    #[test]
    fn parser_rejects_zero_divisor() {
        let input = TEST_OFFICIAL_1.replace("divisible by 23", "divisible by 0");
        assert!(input.parse::<InputData>().is_err());
    }
}
