//! # 2019 day 6 - Universal Orbit Map
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    orbits: HashMap<&'a str, Vec<&'a str>>,
    orbiting: HashMap<&'a str, &'a str>,
    you: &'a str,
    santa: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut orbits = HashMap::new();
        let mut orbiting = HashMap::new();
        let mut you = None;
        let mut santa = None;
        for line in s.lines() {
            let (left, right) = line.split_once(')').unwrap();
            orbits
                .entry(left)
                .and_modify(|v: &mut Vec<&str>| v.push(right))
                .or_insert(vec![right]);
            orbiting.insert(right, left);
            if right == "YOU" {
                you = Some(left);
            } else if right == "SAN" {
                santa = Some(left);
            }
        }
        Ok(Self {
            orbits,
            orbiting,
            you: you.unwrap_or_default(),
            santa: santa.unwrap_or_default(),
        })
    }
}

impl<'a> InputData<'a> {
    fn get_all_orbits(&self, obj: &'a str, seen: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(cache) = seen.get(obj) {
            *cache
        } else if self.orbits.contains_key(obj) {
            let res = self
                .orbits
                .get(obj)
                .unwrap()
                .iter()
                .map(|v| 1 + self.get_all_orbits(v, seen))
                .sum();
            seen.insert(obj, res);
            res
        } else {
            0
        }
    }
    fn solve_part1(&self) -> usize {
        let seen = &mut HashMap::new();
        self.orbits
            .keys()
            .map(|k| self.get_all_orbits(k, seen))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut you_path = Vec::from([self.you]);
        while you_path.last().unwrap().ne(&"COM") {
            you_path.push(self.orbiting.get(you_path.last().unwrap()).unwrap());
        }
        let mut santa_path = Vec::from([self.santa]);
        while santa_path.last().unwrap().ne(&"COM") {
            santa_path.push(self.orbiting.get(santa_path.last().unwrap()).unwrap());
        }
        while you_path.last() == santa_path.last() {
            you_path.pop();
            santa_path.pop();
        }
        you_path.len() + santa_path.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 42);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
