//! # 2019 day 6 - Universal Orbit Map
//!
//! Store the orbits in two formats inside an orbit class - one dict to get all the objects
//! orbiting a certain object, and one 'reversed' variant for Part 2 to get the parent object
//! for each object. From there it's simple recursion to get the answer for Part 1.
//!
//! For Part 2, recurse through the second dict to get the paths from YOU -> COM and SAN -> COM,
//! then find where those two paths meet and get the answer from the remaining length of both paths.
use std::{collections::HashMap, error::Error};

use crate::aoc_util::error::{AocError, OptionExt};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData<'a> {
    orbits: HashMap<&'a str, Vec<&'a str>>,
    orbiting: HashMap<&'a str, &'a str>,
    you: Option<&'a str>,
    santa: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut orbits = HashMap::new();
        let mut orbiting = HashMap::new();
        let mut you = None;
        let mut santa = None;
        for line in s.lines() {
            let (left, right) = line.split_once(')').ctx("line: )")?;
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
            you,
            santa,
        })
    }
}

impl<'a> InputData<'a> {
    fn get_all_orbits(&self, obj: &'a str, seen: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(cache) = seen.get(obj) {
            *cache
        } else if let Some(k) = self.orbits.get(obj) {
            let res = k.iter().map(|v| 1 + self.get_all_orbits(v, seen)).sum();
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

    fn solve_part2(&self) -> Result<usize, AocError> {
        let santa = self.santa.ok_or(AocError::NoSolution)?;
        let you = self.you.ok_or(AocError::NoSolution)?;
        let mut you_path = Vec::from([you]);
        while you_path
            .last()
            .expect("you_path should never be empty")
            .ne(&"COM")
        {
            let parent = *self
                .orbiting
                .get(you_path.last().expect("you_path should not be empty"))
                .expect("orbit chain must reach COM");
            you_path.push(parent);
        }
        let mut santa_path = Vec::from([santa]);
        while santa_path
            .last()
            .expect("santa_path should never be empty")
            .ne(&"COM")
        {
            let parent = *self
                .orbiting
                .get(santa_path.last().expect("santa_path should never be empty"))
                .expect("orbit chain must reach COM");
            santa_path.push(parent);
        }
        while !you_path.is_empty() && !santa_path.is_empty() && you_path.last() == santa_path.last()
        {
            you_path.pop();
            santa_path.pop();
        }
        // Note: we chopped off also the connecting node, so it should have been this +1. But we also want
        // the number of edges, not number of nodes, so it would be number of nodes -1. So it cancels out.
        Ok(you_path.len() + santa_path.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "COM)B
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

    const TEST_OFFICIAL_2: &str = "COM)B
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

    /// YOU's object (B) is an ancestor of SAN's object (A)
    const TEST_CUSTOM_1: &str = "COM)A
A)B
B)YOU
A)SAN";

    /// YOU and SAN are siblings, both attached to B
    const TEST_CUSTOM_2: &str = "COM)A
A)B
B)YOU
B)SAN";

    /// YOU and SAN both attach directly to COM
    const TEST_CUSTOM_3: &str = "COM)YOU
COM)SAN";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 42);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(
            solution_data.solve_part2().unwrap_err(),
            AocError::NoSolution
        );
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 4);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 8);
        assert_eq!(solution_data.solve_part2().unwrap(), 1);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 9);
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }

    #[test]
    fn part1_2_custom_3() {
        let solution_data = InputData::try_from(TEST_CUSTOM_3).unwrap();
        assert_eq!(solution_data.solve_part1(), 2);
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }
}
