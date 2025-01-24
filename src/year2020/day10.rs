//! # 2020 day 10 - Adapter Array
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    adapters: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adapters: Vec<usize> = s.lines().map(|line| line.parse().unwrap()).collect();
        adapters.push(0);
        adapters.sort_unstable();
        let largest = adapters.last().unwrap();
        adapters.push(*largest + 3);
        Ok(Self { adapters })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut diffs = HashMap::new();
        self.adapters.windows(2).for_each(|a| {
            diffs
                .entry(a[1] - a[0])
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
        diffs.get(&1).unwrap() * diffs.get(&3).unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut counts = (0, 0, 1);
        self.adapters.windows(2).for_each(|a| {
            counts = match a[1] - a[0] {
                1 => (counts.1, counts.2, counts.0 + counts.1 + counts.2),
                2 => (counts.2, 0, counts.1 + counts.2),
                3 => (0, 0, counts.2),
                _ => counts,
            }
        });
        counts.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const TEST_DATA_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 7 * 5);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 22 * 10);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 8);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 19208);
    }
}
