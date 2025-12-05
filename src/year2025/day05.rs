//! # 2025 day 5 - Cafeteria
//!
//! ## Part 1
//!
//! Trivial - for every available ID, iterate over the ID ranges and if any range contains
//! the available ID, increment the total and skip to the next available ID.
//!
//! ## Part 2
//!
//! First merge all the overlapping ranges:
//! 1. Put all the input ranges on a queue, and create a list of processed ranges.
//! 2. Pull a range from the queue and check if it overlaps with any of the already
//!    processed ranges. If it does, pull that processed range out of the list and push
//!    the combined range back to the queue.
//! 3. Repeat 2 until the queue is empty.
//!
//! Once the new list of combined ranges is completed, take the total sum of ID:s contained
//! by each range. Note that the ranges are inclusive, so the number of ID:s for a range
//! is 1 + upper_range - lower range.
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    fresh_id_ranges: Vec<(usize, usize)>,
    available_ids: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, a) = s.split_once("\n\n").unwrap();
        let ranges: Vec<(usize, usize)> = r
            .lines()
            .map(|line| {
                let (x, y) = line.split_once('-').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
        Ok(Self {
            fresh_id_ranges: ranges,
            available_ids: a.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut total = 0;
        for a in &self.available_ids {
            for r in &self.fresh_id_ranges {
                if (r.0..=r.1).contains(a) {
                    total += 1;
                    break;
                }
            }
        }
        total
    }

    fn solve_part2(&self) -> usize {
        let mut processed = VecDeque::new();
        let mut queue = VecDeque::from(self.fresh_id_ranges.clone());
        while let Some((r0, r1)) = queue.pop_front() {
            let mut overlap_idx = None;
            for (i, (p0, p1)) in processed.iter().enumerate() {
                if (*p0..=*p1).contains(&r0) || (*p0..=*p1).contains(&r1) || (r0..=r1).contains(p0)
                {
                    overlap_idx = Some(i);
                    break;
                }
            }
            if let Some(i) = overlap_idx {
                let (p0, p1) = processed.remove(i).unwrap();
                queue.push_back((p0.min(r0), p1.max(r1)));
            } else {
                processed.push_back((r0, r1));
            }
        }
        processed.iter().map(|(p0, p1)| 1 + *p1 - *p0).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 14);
    }
}
