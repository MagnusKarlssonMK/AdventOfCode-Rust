//! # 2025 day 8 - Playground
//!
//! Solving both parts with Disjoint-Set/Union-Find algorithm, where the main difference is that
//! for part 1, the stop condition is a certain number of connections, and for part 2 that a circuit
//! contains all elements in the input list.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1(1000).to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point3D {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nbrs: Vec<usize> = s.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Self {
            x: nbrs[0],
            y: nbrs[1],
            z: nbrs[2],
        })
    }
}

impl Point3D {
    fn get_distance(&self, other: &Point3D) -> usize {
        // Note: we only need a sortable distance, the actual value isn't important, so we can skip the sqrt
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct JunctionBox {
    parent: usize,
    count: usize,
}

fn find(junctions: &mut [JunctionBox], mut x: usize) -> usize {
    while junctions[x].parent != x {
        let parent = junctions[x].parent;
        (x, junctions[x].parent) = (parent, junctions[parent].parent);
    }
    x
}

fn union(junctions: &mut [JunctionBox], mut a: usize, mut b: usize) -> usize {
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
    connections: Vec<(usize, usize)>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let junction_boxes: Vec<Point3D> = s
            .lines()
            .map(|line| Point3D::from_str(line).unwrap())
            .collect();
        let mut connections = Vec::new();
        for i in 0..(junction_boxes.len() - 1) {
            for j in (i + 1)..junction_boxes.len() {
                connections.push((i, j));
            }
        }
        connections.sort_unstable_by(|a, b| {
            junction_boxes[a.0]
                .get_distance(&junction_boxes[a.1])
                .cmp(&junction_boxes[b.0].get_distance(&junction_boxes[b.1]))
        });
        Ok(Self {
            junction_boxes,
            connections,
        })
    }
}

impl InputData {
    fn solve_part1(&self, max_connections: usize) -> usize {
        let mut junctions: Vec<_> = (0..self.junction_boxes.len())
            .map(|i| JunctionBox {
                parent: i,
                count: 1,
            })
            .collect();

        for (i, j) in self.connections.iter().take(max_connections) {
            union(&mut junctions, *i, *j);
        }

        junctions.sort_unstable_by(|a, b| b.count.cmp(&a.count));
        junctions[0].count * junctions[1].count * junctions[2].count
    }

    fn solve_part2(&self) -> usize {
        let mut nodes: Vec<_> = (0..self.junction_boxes.len())
            .map(|i| JunctionBox {
                parent: i,
                count: 1,
            })
            .collect();

        for (i, j) in &self.connections {
            if union(&mut nodes, *i, *j) == self.junction_boxes.len() {
                return self.junction_boxes[*i].x * self.junction_boxes[*j].x;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "162,817,812
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
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(10), 40);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 25272);
    }
}
