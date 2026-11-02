//! # 2018 day 7 - The Sum of Its Parts
//!
//! Maps the step characters to vec indices and uses a bitmask for each value to mark
//! which other steps it depends on being done first.
//!
//! Then the check for which step can be taken becomes a simple bit operation.
use crate::aoc_util::error::*;
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data
            .solve_part2(P2_WORKERS, P2_FLAT_COST)?
            .to_string(),
    ))
}

const P2_WORKERS: usize = 5;
const P2_FLAT_COST: usize = 60;

const UPPER_CHAR: char = 'Z';
const LOWER_CHAR: char = 'A';
const OFFSET: u8 = UPPER_CHAR as u8 - LOWER_CHAR as u8;

#[derive(Debug)]
struct InputData {
    need_instructions: Vec<usize>,
    used_characters: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_steps(s: &str) -> Result<(usize, usize), AocError> {
            match s.split(' ').collect::<Vec<_>>().as_slice() {
                [
                    "Step",
                    a,
                    "must",
                    "be",
                    "finished",
                    "before",
                    "step",
                    b,
                    "can",
                    "begin.",
                ] => {
                    let left = a.chars().next().ctx("char a")?;
                    let right = b.chars().next().ctx("char b")?;
                    if !(LOWER_CHAR..=UPPER_CHAR).contains(&left)
                        || !(LOWER_CHAR..=UPPER_CHAR).contains(&right)
                    {
                        Err(AocError::Invalid(s.to_string()))
                    } else {
                        let left = left as u8 - LOWER_CHAR as u8;
                        let right = right as u8 - LOWER_CHAR as u8;
                        Ok((left as usize, right as usize))
                    }
                }
                _ => Err(AocError::Invalid(s.to_string())),
            }
        }
        let mut need_instructions = vec![0; (1 + OFFSET) as usize];
        let mut used_characters = 0;
        for line in s.lines() {
            let (left, right) = extract_steps(line)?;
            need_instructions[right] |= 1 << left;
            used_characters |= 1 << left;
            used_characters |= 1 << right;
        }
        Ok(Self {
            need_instructions,
            used_characters,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<String, AocError> {
        let mut result = Vec::new();
        let mut done = 0;
        let nbr_nodes: usize = self.used_characters.count_ones() as usize;
        'outer: for _ in 0..nbr_nodes {
            // When multiple steps are open, choose in alphabetical order - fulfilled by looping in ascending order
            for i in 0..=OFFSET as usize {
                // Skip characters that either are not present in input, or have already been used
                let mask = 1usize << i;
                if mask & self.used_characters == 0 || mask & done > 0 {
                    continue;
                }
                if self.need_instructions[i] & done == self.need_instructions[i] {
                    done += mask;
                    result.push((i as u8 + LOWER_CHAR as u8) as char);
                    continue 'outer;
                }
            }
            return Err(AocError::NoSolution);
        }
        Ok(result.iter().collect())
    }

    fn solve_part2(&self, nbrof_workers: usize, flat_cost: usize) -> Result<usize, AocError> {
        let mut workers = HashMap::new(); // step character; minutes remaining
        let mut steps = 0;
        let mut done = 0usize;
        let nbr_nodes: usize = self.used_characters.count_ones() as usize;
        'outer: loop {
            // Process all busy workers
            let mut finished = Vec::new();
            let min_worker_steps = *workers.values().min().unwrap_or(&1); // Should only be None the first iteration
            for (k, v) in &mut workers {
                *v -= min_worker_steps;
                if *v == 0 {
                    finished.push(*k);
                }
            }
            // Free any finished workers and mark its character bit as done
            for k in finished {
                workers.remove(&k);
                done += k;
            }
            steps += min_worker_steps - 1;
            if done.count_ones() as usize >= nbr_nodes {
                // We've taken all steps; we are done.
                break;
            }
            steps += 1;
            for i in 0..=OFFSET as usize {
                // Skip characters that either are not present in input, or have already been used or are being worked on
                let mask = 1usize << i;
                if mask & self.used_characters == 0
                    || mask & done > 0
                    || workers.contains_key(&mask)
                {
                    continue;
                }
                if self.need_instructions[i] & done == self.need_instructions[i] {
                    workers.insert(mask, flat_cost + i + 1);
                    if workers.len() == nbrof_workers {
                        // All workers are now busy
                        continue 'outer;
                    }
                }
            }
            if workers.is_empty() {
                return Err(AocError::NoSolution);
            }
        }
        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    /// A takes 1 and B takes 2 ticks; so with B depending on A the run ends at 3.
    const TEST_CUSTOM_1: &str = "Step A must be finished before step B can begin.";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), "CABDFE");
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(2, 0).unwrap(), 15);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), "AB");
        assert_eq!(solution_data.solve_part2(2, 0).unwrap(), 3);
    }
}
