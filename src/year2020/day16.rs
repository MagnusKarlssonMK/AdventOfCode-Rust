//! # 2020 day 16 - Ticket Translation
//!
//! ## Part 1
//!
//! Every value in a nearby ticket must be accepted by at least one field rule. The answer is the
//! sum of all values that no rule accepts.
//!
//! ## Part 2
//!
//! Tickets containing any value that no rule accepts are discarded. For each column, the candidate
//! rules are those that accept every value in that column across all remaining tickets (and our own
//! ticket). A column with exactly one candidate is assigned that rule, and the rule is removed from
//! the other columns; this repeats until every column is assigned. If some columns remain with more
//! than one candidate, the mapping is ambiguous and there is no unique solution. The answer is the
//! product of the values in our ticket whose assigned rule is a departure rule.
//!
//! The solver assumes there is exactly one field rule per ticket column; a different number of
//! rules and columns yields `AocError::NoSolution`. All nearby tickets must have the same number of
//! values as our ticket.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, ops::RangeInclusive, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(Debug)]
struct FieldRule {
    is_departure: bool,
    ranges: [RangeInclusive<u64>; 2],
}

impl FieldRule {
    fn accepts(&self, value: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

fn parse_range(s: &str) -> Result<RangeInclusive<u64>, AocError> {
    let (lower, upper) = s.split_once('-').ctx(format!("range {s:?}"))?;
    let lower = lower.parse()?;
    let upper = upper.parse()?;
    if lower > upper {
        return Err(AocError::Invalid(format!("range {s:?}")));
    }
    Ok(lower..=upper)
}

impl FromStr for FieldRule {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, ranges) = s
            .split_once(": ")
            .ctx(format!("field rule {s:?}"))?;
        let (range1, range2) = ranges
            .split_once(" or ")
            .ctx(format!("field rule {s:?}"))?;
        Ok(Self {
            is_departure: name.starts_with("departure"),
            ranges: [parse_range(range1)?, parse_range(range2)?],
        })
    }
}

struct InputData {
    fieldrules: Vec<FieldRule>,
    your: Vec<u64>,
    nearby: Vec<Vec<u64>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_ticket(s: &str) -> Result<Vec<u64>, AocError> {
            s.split(',')
                .map(|value| value.parse().map_err(AocError::from))
                .collect()
        }

        let mut blocks = s.split("\n\n");

        let fieldrules = blocks
            .next()
            .ctx("fields block")?
            .lines()
            .map(FieldRule::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        if fieldrules.is_empty() {
            return Err(AocError::Invalid("no field rules".to_string()));
        }

        let your = parse_ticket(
            blocks
                .next()
                .ctx("your ticket block")?
                .lines()
                .nth(1)
                .ctx("your ticket line")?,
        )?;

        let nearby = blocks
            .next()
            .ctx("nearby tickets block")?
            .lines()
            .skip(1)
            .map(parse_ticket)
            .collect::<Result<Vec<_>, _>>()?;

        if let Some(ticket) = nearby.iter().find(|ticket| ticket.len() != your.len()) {
            return Err(AocError::Invalid(format!(
                "nearby ticket has {} values, expected {}",
                ticket.len(),
                your.len()
            )));
        }

        Ok(Self {
            fieldrules,
            your,
            nearby,
        })
    }
}

impl InputData {
    fn is_valid_ticket(&self, ticket: &[u64]) -> bool {
        ticket
            .iter()
            .all(|value| self.fieldrules.iter().any(|rule| rule.accepts(*value)))
    }

    fn solve_part1(&self) -> u64 {
        self.nearby
            .iter()
            .flatten()
            .filter(|value| !self.fieldrules.iter().any(|rule| rule.accepts(**value)))
            .sum()
    }

    /// Maps each ticket column to the index of its field rule.
    fn field_assignment(&self) -> Result<Vec<usize>, AocError> {
        if self.fieldrules.len() != self.your.len() {
            return Err(AocError::NoSolution);
        }

        // The assignment must be consistent with every valid nearby ticket and with our own ticket.
        let mut tickets: Vec<&[u64]> = self
            .nearby
            .iter()
            .filter(|ticket| self.is_valid_ticket(ticket))
            .map(|ticket| ticket.as_slice())
            .collect();
        tickets.push(self.your.as_slice());

        let mut candidates: Vec<Vec<usize>> = (0..self.your.len())
            .map(|column| {
                self.fieldrules
                    .iter()
                    .enumerate()
                    .filter(|(_, rule)| tickets.iter().all(|ticket| rule.accepts(ticket[column])))
                    .map(|(index, _)| index)
                    .collect()
            })
            .collect();

        if candidates.iter().any(Vec::is_empty) {
            return Err(AocError::NoSolution);
        }

        let mut assignment = vec![None; self.your.len()];
        let mut remaining = self.your.len();

        while remaining > 0 {
            let mut next = None;
            for (column, candidate_list) in candidates.iter().enumerate() {
                if assignment[column].is_none() && candidate_list.len() == 1 {
                    next = Some((column, candidate_list[0]));
                    break;
                }
            }
            let Some((column, field)) = next else {
                // No forced assignment remains, so the mapping is ambiguous.
                return Err(AocError::NoSolution);
            };

            assignment[column] = Some(field);
            remaining -= 1;

            for (other, candidate_list) in candidates.iter_mut().enumerate() {
                if assignment[other].is_none() {
                    candidate_list.retain(|candidate| *candidate != field);
                    if candidate_list.is_empty() {
                        return Err(AocError::NoSolution);
                    }
                }
            }
        }

        Ok(assignment
            .into_iter()
            .map(|field| field.expect("every column is assigned"))
            .collect())
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let assignment = self.field_assignment()?;
        let mut product = 1u64;
        for (column, field) in assignment.into_iter().enumerate() {
            if self.fieldrules[field].is_departure {
                product = product
                    .checked_mul(self.your[column])
                    .ok_or(AocError::NoSolution)?;
            }
        }
        Ok(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const TEST_OFFICIAL_2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    /// Has an invalid nearby ticket (100) plus two departure fields, so part 2 exercises both the
    /// filtering and the departure product.
    const TEST_CUSTOM_1: &str = "departure location: 1-3 or 5-7
departure station: 20-22 or 24-25
class: 4-6 or 8-9
row: 10-12 or 14-15

your ticket:
2,5,11,21

nearby tickets:
2,5,11,21
100,4,12,21";

    /// Two identical fields make the mapping impossible to deduce.
    const TEST_CUSTOM_2: &str = "a: 0-5 or 10-15
b: 0-5 or 10-15

your ticket:
1,1

nearby tickets:
1,1";

    /// A unique mapping that only becomes forced after eliminating an assigned rule.
    const TEST_CUSTOM_3: &str = "departure a: 1-1 or 2-2
b: 2-2 or 3-3
departure c: 3-3 or 4-4

your ticket:
1,2,3

nearby tickets:
1,2,3";

    /// The second nearby ticket has two invalid values; part 1 must sum both.
    const TEST_CUSTOM_4: &str = "a: 1-3 or 5-7
b: 8-10 or 12-14

your ticket:
1,8

nearby tickets:
1,8
4,11";

    /// Both columns only accept the same rule, so no bijection exists.
    const TEST_CUSTOM_5: &str = "a: 1-1 or 1-1
b: 2-2 or 2-2

your ticket:
1,1

nearby tickets:
1,1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 71);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(1));
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 100);
        assert_eq!(solution_data.solve_part2(), Ok(42));
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn part2_custom_3() {
        let solution_data: InputData = TEST_CUSTOM_3.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(3));
    }

    #[test]
    fn part1_custom_4() {
        let solution_data: InputData = TEST_CUSTOM_4.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 15);
    }

    #[test]
    fn part2_custom_5() {
        let solution_data: InputData = TEST_CUSTOM_5.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn malformed_field_rule_is_error() {
        assert!("no colon".parse::<FieldRule>().is_err());
        assert!("a: 3-1 or 5-7".parse::<FieldRule>().is_err());
        assert!("a: 1-3 or nope".parse::<FieldRule>().is_err());
    }

    #[test]
    fn ticket_width_mismatch_is_error() {
        let input = "a: 1-1 or 1-1

your ticket:
1,2

nearby tickets:
1";
        assert!(input.parse::<InputData>().is_err());
    }
}
