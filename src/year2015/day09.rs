//! # 2015 day 9 - All in a Single Night
use crate::aoc_util::error::AocError;
use std::{collections::HashMap, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

/// Advances a slice to the next lexicographic permutation, returning `false`
/// when it is already the last one.
fn next_permutation(values: &mut [usize]) -> bool {
    let Some(i) = values.windows(2).rposition(|w| w[0] < w[1]) else {
        return false;
    };
    let j = values[i + 1..]
        .iter()
        .rposition(|&v| v > values[i])
        .map(|offset| i + 1 + offset)
        .expect("a larger value exists after the descent");
    values.swap(i, j);
    values[i + 1..].reverse();
    true
}

fn city_id(cities: &mut HashMap<String, usize>, name: &str) -> usize {
    if let Some(&id) = cities.get(name) {
        id
    } else {
        let id = cities.len();
        cities.insert(name.to_string(), id);
        id
    }
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
            if tokens.len() != 5 || tokens[1] != "to" {
                return Err(AocError::Invalid(line.to_string()));
            }
            let from = city_id(&mut cities, tokens[0]);
            let to = city_id(&mut cities, tokens[2]);
            let d: usize = tokens[4].parse()?;
            distances.insert((from, to), d);
            distances.insert((to, from), d);
        }
        if cities.len() < 2 {
            return Err(AocError::Invalid("at least two cities required".into()));
        }
        Ok(Self { cities, distances })
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut p1 = usize::MAX;
        let mut p2 = 0;

        let mut route: Vec<usize> = self.cities.values().copied().collect();
        route.sort_unstable();
        // Note: skip routes where the first city is greater than the last, to avoid calculating
        // also the reverse route.
        loop {
            if route[0] < route[route.len() - 1] {
                let mut newlen = 0;
                let mut complete = true;
                for i in 0..route.len() - 1 {
                    match self.distances.get(&(route[i], route[i + 1])) {
                        Some(d) => newlen += *d,
                        None => {
                            complete = false;
                            break;
                        }
                    }
                }
                if complete {
                    p1 = p1.min(newlen);
                    p2 = p2.max(newlen);
                }
            }
            if !next_permutation(&mut route) {
                break;
            }
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
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 605);
        assert_eq!(p2, 982);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 6);
        assert_eq!(p2, 9);
    }

    #[test]
    fn part1_2_no_solution() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("disconnected tree".to_string()));
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "A B C = 1",  // missing 'to'
            "A to A = 1", // only one city
            "A to B = x", // bad distance
        ] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
    }
}
