use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData<'a> {
    replacements: HashMap<String, Vec<String>>,
    molecule: &'a str,
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        let (r, molecule) = input.split_once("\n\n").unwrap();
        let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
        for line in r.lines() {
            let (left, right) = line.split_once(" => ").unwrap();
            replacements
                .entry(left.to_string())
                .and_modify(|v| v.push(right.to_string()))
                .or_insert(vec![right.to_string()]);
        }
        Self {
            replacements,
            molecule,
        }
    }

    fn solve_part1(&self) -> usize {
        let mut altered: HashSet<String> = HashSet::new();
        for (replaced, v) in self.replacements.iter() {
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
        let testdata = String::from(
            "H => HO
H => OH
O => HH

HOH",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from(
            "H => HO
H => OH
O => HH

HOHOHO",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 7);
    }
}
