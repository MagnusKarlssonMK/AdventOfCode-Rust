//! # 2017 day 14 - Disk Defragmentation
//!
//! Re-use the knot hasher from day 10 (util.knot) to generate a 128x128 grid of ones and zeroes, which is
//! turned into an adjacency graph of neighboring ones. Part 1 counts the "1" cells, and part 2 walks the
//! graph to count the connected groups.
use crate::aoc_util::{
    error::*,
    grid::Grid,
    point::{NEIGHBORS_STRAIGHT, Point},
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

use super::knothash::*;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const GRID_SIZE: usize = 128;

struct InputData {
    graph: HashMap<Point, HashSet<Point>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::new(GRID_SIZE, GRID_SIZE, '0');
        for y in 0..GRID_SIZE as i32 {
            let seed = s.to_string() + "-" + &y.to_string();
            let mut knot = u128::from_str_radix(&get_knot_hash(&seed, 256), 16)?;
            for x in (0..GRID_SIZE as i32).rev() {
                if knot & 1 == 1 {
                    grid.set_point(&Point::new(x, y), '1');
                }
                knot >>= 1;
            }
        }
        let mut graph = HashMap::new();
        for (i, c) in grid.elements.iter().enumerate() {
            if *c == '1' {
                let p = grid.get_point(i);
                graph.insert(p, HashSet::new()); // Insert with an empty list first; a node might not have any neighbors at all
                for n in NEIGHBORS_STRAIGHT
                    .iter()
                    .map(|d| *d + p)
                    .filter(|n| grid.get_element(n) == Some('1'))
                {
                    graph.entry(p).or_default().insert(n);
                }
            }
        }
        Ok(Self { graph })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.graph.keys().len()
    }

    fn solve_part2(&self) -> usize {
        let mut groups = 0;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        for square in self.graph.keys() {
            if seen.contains(square) {
                continue;
            }
            seen.insert(square);
            queue.push_back(square);
            while let Some(v) = queue.pop_front() {
                for n in self.graph[v].iter() {
                    if !seen.contains(n) {
                        queue.push_back(n);
                        seen.insert(n);
                    }
                }
            }
            groups += 1
        }
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL: &str = "flqrgnkx";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL).unwrap();
        assert_eq!(solution_data.solve_part1(), 8108);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL).unwrap();
        assert_eq!(solution_data.solve_part2(), 1242);
    }
}
