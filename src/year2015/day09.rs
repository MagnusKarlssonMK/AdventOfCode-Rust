//! # 2015 day 9 - All in a Single Night
use crate::aoc_util::error::AocError;
use itertools::Itertools;
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    cities: HashMap<String, usize>,
    distances: HashMap<(usize, usize), usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        let mut cities: HashMap<String, usize> = HashMap::new();
        for line in s.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() != 5 {
                return Err(AocError::Invalid(line.to_string()));
            }
            let f = tokens[0].to_string();
            let t = tokens[2].to_string();
            let d: usize = tokens[4].parse()?;
            let mut len = cities.len();
            if !cities.contains_key(&f) {
                cities.insert(f.clone(), len);
                len += 1;
            }
            if !cities.contains_key(&t) {
                cities.insert(t.clone(), len);
            }
            distances.insert((*cities.get(&f).unwrap(), *cities.get(&t).unwrap()), d);
            distances.insert((*cities.get(&t).unwrap(), *cities.get(&f).unwrap()), d);
        }
        Ok(Self { cities, distances })
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut p1 = usize::MAX;
        let mut p2 = 0;

        // Note: filter first < last to avoid calculating also the reverse route
        'outer: for route in self
            .cities
            .values()
            .permutations(self.cities.len())
            .filter(|r| r.first().unwrap() < r.last().unwrap())
        {
            let mut newlen = 0;
            for i in 0..route.len() - 1 {
                if let Some(d) = self.distances.get(&(*route[i], *route[i + 1])) {
                    newlen += *d;
                } else {
                    continue 'outer;
                }
            }
            p1 = p1.min(newlen);
            p2 = p2.max(newlen);
        }
        if p1 == usize::MAX {
            Err(AocError::Invalid("disconnected tree".to_string()))
        } else {
            Ok((p1, p2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    const TEST_CUSTOM_1: &str = "A to B = 1
B to C = 2
C to D = 3
A to D = 4";

    const TEST_CUSTOM_2: &str = "A to B = 1
C to D = 2";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 605);
        assert_eq!(p2, 982);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 6);
        assert_eq!(p2, 9);
    }

    #[test]
    fn part1_2_no_solution() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("disconnected tree".to_string()));
    }
}
