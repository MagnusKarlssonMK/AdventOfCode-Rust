//! # 2020 day 16 - Ticket Translation
//!
//! Kind of like Sudoku.
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug)]
struct FieldRule {
    is_departure: bool,
    range1: (usize, usize),
    range2: (usize, usize),
}

impl FromStr for FieldRule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let is_departure = left.starts_with("departure");
        let (r1, r2) = right.split_once(" or ").unwrap();
        let r1 = r1.split_once('-').unwrap();
        let r2 = r2.split_once('-').unwrap();
        Ok(Self {
            is_departure,
            range1: (r1.0.parse().unwrap(), r1.1.parse().unwrap()),
            range2: (r2.0.parse().unwrap(), r2.1.parse().unwrap()),
        })
    }
}

impl FieldRule {
    fn is_in_range(&self, n: &usize) -> bool {
        (self.range1.0..=self.range1.1).contains(n) || (self.range2.0..=self.range2.1).contains(n)
    }
}

struct InputData {
    fieldrules: Vec<FieldRule>,
    your: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let fields = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| FieldRule::from_str(line).unwrap())
            .collect();
        let your = parts
            .next()
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let nearby = parts
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();
        Ok(Self {
            fieldrules: fields,
            your,
            nearby,
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut invalid = 0;
        let mut valid = Vec::new();
        'outer: for ticket in &self.nearby {
            for nbr in ticket {
                if !self.fieldrules.iter().any(|f| f.is_in_range(nbr)) {
                    invalid += nbr;
                    continue 'outer;
                }
            }
            valid.push(ticket);
        }

        let nbrcols = self.nearby.first().unwrap().len();
        // Create vec of possible rule indices for each position, wrapped with the index of the position to keep track after sorting
        let mut possible: Vec<(usize, Vec<usize>)> = (0..nbrcols)
            .map(|i| {
                (
                    i,
                    self.fieldrules
                        .iter()
                        .enumerate()
                        .filter(|(_, f)| valid.iter().all(|n| f.is_in_range(n.get(i).unwrap())))
                        .map(|(idx, _)| idx)
                        .collect(),
                )
            })
            .collect();
        // Sort the vec by number of possible values, to minimize the number of possible paths when traversing
        possible.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));
        // Ugly! - start with a vector of some invalid value to avoid getting false matches on 'contains' check later in the search
        let mut path = vec![nbrcols + 1; nbrcols];
        let mut queue = VecDeque::from([(path.clone(), 0)]);
        while let Some((current, nbr)) = queue.pop_front() {
            if nbr == nbrcols {
                path = current;
                break;
            }
            for nxt in possible.get(nbr).unwrap().1.iter() {
                if !current.contains(nxt) {
                    let mut n = current.clone();
                    n[possible.get(nbr).unwrap().0] = *nxt;
                    queue.push_back((n, nbr + 1));
                }
            }
        }
        let departures = path
            .iter()
            .enumerate()
            .filter(|(_, &p)| self.fieldrules.get(p).unwrap().is_departure)
            .map(|(i, _)| self.your.get(i).unwrap())
            .product();
        (invalid, departures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 71);
        assert_eq!(p2, 1);
    }
}
