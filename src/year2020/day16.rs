//! # 2020 day 16 - Ticket Translation
use std::{collections::{HashSet, VecDeque}, error::Error, str::FromStr};

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
    fields: Vec<FieldRule>,
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
            fields,
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
                if !self.fields.iter().any(|f| f.is_in_range(nbr)) {
                    invalid += nbr;
                    continue 'outer;
                }
            }
            valid.push(ticket);
        }

        let nbrcols = self.nearby.first().unwrap().len();
        let mut nbrsets: Vec<HashSet<usize>> = Vec::with_capacity(nbrcols);
        for i in 0..nbrcols {
            nbrsets.push(HashSet::from_iter(self.nearby.iter().map(|n| *n.get(i).unwrap())));
        }
        let possible: Vec<Vec<usize>> = nbrsets.iter().map(|nbrset| self.fields.iter().filter(|field| nbrset.iter().all(|nbr| field.is_in_range(nbr))).enumerate().map(|(i, _)| i).collect::<Vec<usize>>()).collect();
        let mut possible: Vec<_> = possible.iter().enumerate().map(|(i, p)| (i, p)).collect();
        possible.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));
        let mut queue = VecDeque::from_iter(possible.first().unwrap().1);
        //let mut paths = Vec::new();
        while let Some(current) = queue.pop_front() {break;}
        (invalid, 1)
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
