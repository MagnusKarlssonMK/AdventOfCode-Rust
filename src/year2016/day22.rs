//! # 2016 day 22 - Grid Computing
//!
//! ## Part 1
//!
//! Parse the data into vector of the coordinates and node data. Then count the number
//! of pairs where a non-empty node's data would fit in another node's available space.
//!
//! ## Part 2
//!
//! Convert the input vector to a hashmap to be able to lookup per x-y point.
//! The empty node (used == 0) is the only node that can move data around, so first
//! BFS the empty node to the position just left of the goal node G in the top-right
//! corner, treating nodes whose used is larger than the empty node's capacity as walls.
//! Once there, moving G one column to the left takes a single move; each further
//! column takes 5 moves (the empty node has to loop around G), so add
//! 1 + 5 * (max_x - 1) for walking G to the top-left.
use crate::aoc_util::{error::*, point::*};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

impl Point {
    fn parse_path(s: &str) -> Result<Point, AocError> {
        let (_, right) = s.split_once("-x").ctx("path separator -x")?;
        let (x, y) = right.split_once("-y").ctx("path separator -y")?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct NodeStats {
    size: usize,
    used: usize,
    avail: usize,
}

impl FromStr for NodeStats {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .split_ascii_whitespace()
            .map(|p| p.trim_end_matches(['T', '%']))
            .collect();
        match parts.as_slice() {
            [size, used, avail, _] => Ok(NodeStats {
                size: size.parse()?,
                used: used.parse()?,
                avail: avail.parse()?,
            }),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct InputData {
    filesystem: Vec<(Point, NodeStats)>,
    x_max: i32,
    /// (position, capacity)
    zero: (Point, usize),
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut filesystem = Vec::new();
        let mut zero = None;
        let mut x_max = 0;
        for line in s.lines().skip(2) {
            let (left, right) = line.split_once(' ').ctx("space separator")?;
            let p = Point::parse_path(left)?;
            let n = NodeStats::from_str(right.trim())?;
            x_max = x_max.max(p.x);
            if n.used == 0 {
                zero = Some((p, n.size));
            }
            filesystem.push((p, n));
        }
        Ok(Self {
            filesystem,
            x_max,
            zero: zero.ok_or(AocError::Invalid("no zero-node".to_string()))?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut count = 0;
        let len = self.filesystem.len();
        for i in 0..len {
            let a = &self.filesystem[i].1;
            for j in (i + 1)..len {
                let b = &self.filesystem[j].1;
                if (a.used > 0 && a.used <= b.avail) || (b.used > 0 && b.used <= a.avail) {
                    count += 1;
                }
            }
        }
        count
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        let adj: HashMap<_, _> = self.filesystem.iter().map(|(p, n)| (*p, *n)).collect();
        // BFS the empty node to the left of G, avoiding nodes too full for it to absorb.
        let node_g_left = Point::new(self.x_max - 1, 0);
        let mut seen = HashSet::from([self.zero.0]);
        let mut queue = VecDeque::from([(self.zero.0, 0)]);
        while let Some((next_node, steps)) = queue.pop_front() {
            if next_node == node_g_left {
                return Ok(steps + 1 + 5 * ((self.x_max as usize).saturating_sub(1)));
            }
            for n in NEIGHBORS_STRAIGHT.iter().map(|d| next_node + *d) {
                if let Some(v) = adj.get(&n)
                    && !seen.contains(&n)
                    && v.used <= self.zero.1
                {
                    seen.insert(n);
                    queue.push_back((n, steps + 1));
                }
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 7);
    }

    /// The full node doesn't fit in the empty node, so the only "fit" is the empty
    /// node's data into the full node - which doesn't count since a source has to
    /// be non-empty.
    const TEST_EMPTY_NOT_SOURCE: &str = "root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     10T    0T    10T    0%
/dev/grid/node-x1-y0     12T   12T     0T  100%";

    #[test]
    fn part1_empty_node_not_a_source() {
        let solution_data = InputData::from_str(TEST_EMPTY_NOT_SOURCE).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    /// All non-empty nodes are too full for the empty node to absorb, so it can
    /// never reach the position left of the goal.
    const TEST_UNREACHABLE_GOAL: &str = "root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y1     10T    0T    10T    0%
/dev/grid/node-x0-y0     20T   20T     0T  100%
/dev/grid/node-x1-y0     20T   20T     0T  100%
/dev/grid/node-x2-y0     20T   20T     0T  100%";

    #[test]
    fn part2_unreachable_goal() {
        let solution_data = InputData::from_str(TEST_UNREACHABLE_GOAL).unwrap();
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn parser_successful() {
        let testdata = "root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y6     94T   64T    30T   68%
/dev/grid/node-x0-y7     87T   0T    50T   82%";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.filesystem[0].0, Point::new(0, 6));
        assert_eq!(solution_data.filesystem[0].1.size, 94);
        assert_eq!(solution_data.filesystem[0].1.used, 64);
        assert_eq!(solution_data.filesystem[0].1.avail, 30);
        assert_eq!(solution_data.x_max, 0);
        assert_eq!(solution_data.zero, (Point::new(0, 7), 87));
    }

    #[test]
    fn parser_no_zero() {
        let testdata = "root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y6     94T   64T    30T   68%
/dev/grid/node-x0-y7     87T   30T    50T   82%";
        let err = InputData::from_str(testdata).unwrap_err();
        assert_eq!(err, AocError::Invalid("no zero-node".to_string()))
    }
}
