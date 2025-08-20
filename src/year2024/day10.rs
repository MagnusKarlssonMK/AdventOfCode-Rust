//! # 2024 day 10 - Hoof It
//!
//! Solves both parts in one go by using BFS to find trails, but with the
//! simplification that there is no need to keep track of previously visited
//! nodes. Since the only valid neighbors have a value of +1, all paths
//! generated during the traversal are guaranteed to be unique. Thus the answer
//! to part 2 is given by the number of times the number 9 is found, and the
//! answer to part 1 by throwing all the points found for each trailhead into
//! a hashset and then calculating its length.
use crate::aoc_util::{grid::*, point::*};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    map: Grid,
    trailheads: Vec<Point>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = Grid::parse(s);
        let trailheads = map
            .elements
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == '0')
            .map(|(i, _)| Point::new((i % map.x_max) as i32, (i / map.y_max) as i32))
            .collect();
        Ok(Self { map, trailheads })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut totalscore = 0;
        let mut totalrating = 0;
        for head in &self.trailheads {
            let mut peaks = HashSet::new();
            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(*head);
            while let Some(current) = queue.pop_front() {
                if self.map.get_uint_element(&current).unwrap() == 9 {
                    peaks.insert(current);
                    totalrating += 1;
                } else {
                    for neighbor in NEIGHBORS_STRAIGHT.map(|dir| dir + current) {
                        if let Some(neighbor_val) = self.map.get_uint_element(&neighbor)
                            && neighbor_val == self.map.get_uint_element(&current).unwrap() + 1
                        {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            totalscore += peaks.len();
        }
        (totalscore, totalrating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts1_2_example_1() {
        let testdata = "0123
1234
8765
9876";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1);
    }

    #[test]
    fn parts1_2_example_2() {
        let testdata = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 36);
        assert_eq!(p2, 81);
    }
}
