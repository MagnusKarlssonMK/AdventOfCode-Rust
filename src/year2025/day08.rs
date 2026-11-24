//! # 2025 day 8 - Playground
//!
//! Solving both parts with Disjoint-Set/Union-Find algorithm, where the main difference is that
//! for part 1, the stop condition is a certain number of connections, and for part 2 that a circuit
//! contains all elements in the input list.
//!
//! The squared distance of every pair is precomputed once and sorted, so neither solver needs to
//! recompute distances during sorting or traversal.
use crate::aoc_util::{error::AocError, point::Point3D};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1(1000).to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

impl Point3D {
    fn get_distance(&self, other: &Point3D) -> u64 {
        // Note: we only need a sortable distance, the actual value isn't important, so we can skip the sqrt
        i64::from(self.x).abs_diff(i64::from(other.x)).pow(2)
            + i64::from(self.y).abs_diff(i64::from(other.y)).pow(2)
            + i64::from(self.z).abs_diff(i64::from(other.z)).pow(2)
    }
}

struct JunctionBox {
    parent: usize,
    count: u64,
}

fn find(junctions: &mut [JunctionBox], mut x: usize) -> usize {
    while junctions[x].parent != x {
        let parent = junctions[x].parent;
        (x, junctions[x].parent) = (parent, junctions[parent].parent);
    }
    x
}

fn union(junctions: &mut [JunctionBox], mut a: usize, mut b: usize) -> u64 {
    a = find(junctions, a);
    b = find(junctions, b);

    if a != b {
        if junctions[a].count < junctions[b].count {
            (a, b) = (b, a);
        }
        junctions[b].parent = a;
        junctions[a].count += junctions[b].count;
    }
    junctions[a].count
}

struct InputData {
    junction_boxes: Vec<Point3D>,
    connections: Vec<(u64, usize, usize)>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let junction_boxes: Vec<Point3D> = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        let n = junction_boxes.len();
        if n < 3 {
            return Err(AocError::Invalid(
                "expected at least three junction boxes".into(),
            ));
        }
        let mut connections = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let distance = junction_boxes[i].get_distance(&junction_boxes[j]);
                connections.push((distance, i, j));
            }
        }
        connections.sort_unstable();
        Ok(Self {
            junction_boxes,
            connections,
        })
    }
}

impl InputData {
    fn solve_part1(&self, max_connections: usize) -> u64 {
        let mut junctions: Vec<_> = (0..self.junction_boxes.len())
            .map(|i| JunctionBox {
                parent: i,
                count: 1,
            })
            .collect();

        for &(_, i, j) in self.connections.iter().take(max_connections) {
            union(&mut junctions, i, j);
        }

        junctions.sort_unstable_by_key(|a| std::cmp::Reverse(a.count));
        junctions.iter().take(3).map(|a| a.count).product()
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let mut nodes: Vec<_> = (0..self.junction_boxes.len())
            .map(|i| JunctionBox {
                parent: i,
                count: 1,
            })
            .collect();

        for &(_, i, j) in &self.connections {
            if union(&mut nodes, i, j) == self.junction_boxes.len() as u64 {
                return Ok(self.junction_boxes[i].x as u64 * self.junction_boxes[j].x as u64);
            }
        }
        Err(AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(10), 40);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(25272));
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "1,2\n3,4,5\n6,7,8",
            "1,2,three\n4,5,6\n7,8,9",
            "1,2,3,4\n5,6,7\n8,9,10",
            "1,2,3\n4,5,6",
        ] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
    }

    #[test]
    fn part2_no_solution() {
        let data = InputData {
            junction_boxes: vec![
                Point3D::new(0, 0, 0),
                Point3D::new(1, 0, 0),
                Point3D::new(0, 1, 0),
            ],
            connections: vec![],
        };
        assert_eq!(data.solve_part2(), Err(AocError::NoSolution));
    }
}
