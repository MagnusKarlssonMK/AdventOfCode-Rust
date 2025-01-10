//! # 2015 day 19 - Medicine for Rudolph
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    replacements: HashMap<String, Vec<String>>,
    molecule: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (r, molecule) = s.split_once("\n\n").unwrap();
        let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
        for line in r.lines() {
            let (left, right) = line.split_once(" => ").unwrap();
            replacements
                .entry(left.to_string())
                .and_modify(|v| v.push(right.to_string()))
                .or_insert(vec![right.to_string()]);
        }
        Ok(Self {
            replacements,
            molecule,
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        let mut altered: HashSet<String> = HashSet::new();
        for (replaced, v) in &self.replacements {
            for replacement in v.iter() {
                for (i, _) in self.molecule.match_indices(replaced) {
                    let j = i + replaced.len();
                    altered
                        .insert(self.molecule[..i].to_string() + replacement + &self.molecule[j..]);
                }
            }
        }
        altered.len()
    }

    fn solve_part2(&self) -> usize {
        let elements = self
            .molecule
            .chars()
            .filter(char::is_ascii_uppercase)
            .count();
        let rn = self.molecule.matches("Rn").count();
        let ar = self.molecule.matches("Ar").count();
        let y = self.molecule.matches('Y').count();
        elements - ar - rn - 2 * y - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "H => HO
H => OH
O => HH

HOH";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "H => HO
H => OH
O => HH

HOHOHO";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }
}
