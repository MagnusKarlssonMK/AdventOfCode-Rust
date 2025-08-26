//! # 2023 day 12 - Hot Springs
//!
//! Pretty much a straight transfer of my ugly old Python solution.
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct SpringRow {
    springs: String,
    group_sizes: Vec<usize>,
}

impl FromStr for SpringRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, group_sizes) = s.split_once(' ').unwrap();
        Ok(Self {
            springs: springs.to_string(),
            group_sizes: group_sizes.split(',').map(|c| c.parse().unwrap()).collect(),
        })
    }
}

impl SpringRow {
    fn calculate_combinations(&self, seen: &mut HashMap<SpringRow, usize>) -> usize {
        if self.group_sizes.is_empty() {
            return if self.springs.contains("#") { 0 } else { 1 };
        }
        if self.springs.len() + 1 < self.group_sizes.iter().sum::<usize>() + self.group_sizes.len()
        {
            return 0;
        }
        let is_substrings = self.springs[0..self.group_sizes[0]].contains(".");
        if self.springs.len() == self.group_sizes[0] {
            if is_substrings { 0 } else { 1 }
        } else {
            let can_use = !is_substrings
                && self.group_sizes[0] <= self.springs.len()
                && self.springs.chars().nth(self.group_sizes[0]).unwrap() != '#';
            if self.springs.starts_with('#') {
                if !can_use {
                    return 0;
                } else {
                    let next = SpringRow {
                        springs: self.springs[self.group_sizes[0] + 1..]
                            .trim_start_matches('.')
                            .to_string(),
                        group_sizes: self.group_sizes[1..].to_vec(),
                    };
                    if let Some(v) = seen.get(&next) {
                        return *v;
                    } else {
                        let v = next.calculate_combinations(seen);
                        seen.insert(next, v);
                        return v;
                    }
                }
            }
            let skip = SpringRow {
                springs: self.springs[1..].trim_start_matches('.').to_string(),
                group_sizes: self.group_sizes.to_vec(),
            };
            let skip_v = if let Some(v) = seen.get(&skip) {
                *v
            } else {
                let v = skip.calculate_combinations(seen);
                seen.insert(skip, v);
                v
            };
            if !can_use {
                return skip_v;
            }
            let next = SpringRow {
                springs: self.springs[self.group_sizes[0] + 1..]
                    .trim_start_matches('.')
                    .to_string(),
                group_sizes: self.group_sizes[1..].to_vec(),
            };
            let next_v = if let Some(v) = seen.get(&next) {
                *v
            } else {
                let v = next.calculate_combinations(seen);
                seen.insert(next, v);
                v
            };
            skip_v + next_v
        }
    }
}

struct InputData {
    rows: Vec<SpringRow>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s
                .lines()
                .map(|line| SpringRow::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut seen: HashMap<SpringRow, usize> = HashMap::new();
        self.rows
            .iter()
            .map(|row| row.calculate_combinations(&mut seen))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut seen: HashMap<SpringRow, usize> = HashMap::new();
        self.rows
            .iter()
            .map(|row| {
                let mut extended_row = row.clone();
                for _ in 0..4 {
                    extended_row.springs.push('?');
                    extended_row.springs.push_str(&row.springs);
                    extended_row.group_sizes.extend(row.group_sizes.iter());
                }
                extended_row.calculate_combinations(&mut seen)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "???.### 1,1,3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_2() {
        let testdata = ".??..??...?##. 1,1,3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "?#?#?#?#?#?#?#? 1,3,1,6";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "????.#...#... 4,1,1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_5() {
        let testdata = "????.######..#####. 1,6,5";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_6() {
        let testdata = "?###???????? 3,2,1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "???.### 1,1,3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = ".??..??...?##. 1,1,3";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 16384);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "?#?#?#?#?#?#?#? 1,3,1,6";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_4() {
        let testdata = "????.#...#... 4,1,1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 16);
    }

    #[test]
    fn part2_example_5() {
        let testdata = "????.######..#####. 1,6,5";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 2500);
    }

    #[test]
    fn part2_example_6() {
        let testdata = "?###???????? 3,2,1";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 506250);
    }
}
