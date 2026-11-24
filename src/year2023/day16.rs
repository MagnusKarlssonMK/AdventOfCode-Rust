//! # 2023 day 16 - The Floor Will Be Lava
//!
//! Creates a map of connections between all splitters/mirrors to avoid having to walk step by step
//! every time.
use crate::aoc_util::{error::AocError, grid::Grid, point::*};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

type Edge = (Point, Vec<Point>); // <Next point, new directions>

struct InputData {
    nodes: HashMap<(Point, Point), Edge>, // (point, direction) -> next point and new directions
    x_max: i32,
    y_max: i32,
}

fn bounces(c: &char, direction: &Point) -> bool {
    match *direction {
        UP | DOWN => *c == '/' || *c == '\\' || *c == '-',
        LEFT | RIGHT => *c == '/' || *c == '\\' || *c == '|',
        _ => false,
    }
}

fn get_next_directions(c: &char, direction: &Point) -> Vec<Point> {
    match *direction {
        UP => match *c {
            '-' => vec![LEFT, RIGHT],
            '/' => vec![RIGHT],
            '\\' => vec![LEFT],
            _ => vec![],
        },
        DOWN => match *c {
            '-' => vec![LEFT, RIGHT],
            '/' => vec![LEFT],
            '\\' => vec![RIGHT],
            _ => vec![],
        },
        LEFT => match *c {
            '|' => vec![UP, DOWN],
            '/' => vec![DOWN],
            '\\' => vec![UP],
            _ => vec![],
        },
        RIGHT => match *c {
            '|' => vec![UP, DOWN],
            '/' => vec![UP],
            '\\' => vec![DOWN],
            _ => vec![],
        },
        _ => vec![],
    }
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let mut nodes = HashMap::new();
        // Find every point containing a bouncer or splitter, and create a map of their closest neighbor in each direction
        let mut points: Vec<Point> = grid
            .enumerate_points()
            .filter(|(_, e)| **e != '.')
            .map(|(p, _)| p)
            .collect();
        // Also insert all starting points one step outside the grid in each direction
        for x in 0..grid.x_max as i32 {
            points.push(Point::new(x, -1));
            points.push(Point::new(x, grid.y_max as i32));
        }
        for y in 0..grid.y_max as i32 {
            points.push(Point::new(-1, y));
            points.push(Point::new(grid.x_max as i32, y));
        }
        for p in &points {
            // Find the closest neighbor in each direction that splits or bounces the light
            'neighbors: for d in &NEIGHBORS_STRAIGHT {
                let mut next = *p + *d;
                while let Some(n) = grid.get_element(&next) {
                    if bounces(&n, d) {
                        nodes.insert((*p, *d), (next, get_next_directions(&n, d)));
                        continue 'neighbors;
                    }
                    next += *d;
                }
                // We reached the edge of the grid without hitting any bouncers
                // Take a step back. If we never moved, this node is already on the edge in this
                // direction, so no edge is recorded and the light simply leaves the grid.
                next -= *d;
                if next != *p {
                    nodes.insert((*p, *d), (next, Vec::new()));
                }
            }
        }
        Ok(Self {
            nodes,
            x_max: grid.x_max as i32,
            y_max: grid.y_max as i32,
        })
    }
}

impl InputData {
    /// Counts tiles reached by a beam starting at `(point, direction)`, following precomputed
    /// edges until it leaves the grid or revisits a point/direction pair.
    fn energized_from(&self, start: (Point, Point)) -> usize {
        let mut energized: HashSet<Point> = HashSet::new();
        let mut seen: HashSet<(Point, Point)> = HashSet::new();
        let mut queue = VecDeque::from([start]);

        while let Some((p, d)) = queue.pop_front() {
            if seen.contains(&(p, d)) {
                continue;
            }
            seen.insert((p, d));
            if let Some((next_p, next_d)) = self.nodes.get(&(p, d)) {
                let mut tile = p + d;
                loop {
                    energized.insert(tile);
                    if tile == *next_p {
                        break;
                    } else {
                        tile += d;
                    }
                }
                for n_d in next_d {
                    queue.push_back((*next_p, *n_d));
                }
            }
        }
        energized.len()
    }

    fn solve_part1(&self) -> usize {
        self.energized_from((Point::new(-1, 0), RIGHT))
    }

    fn solve_part2(&self) -> usize {
        let mut starting_points = Vec::new();
        // Also insert all starting points one step outside the grid in each direction
        for x in 0..self.x_max {
            starting_points.push((Point::new(x, -1), DOWN));
            starting_points.push((Point::new(x, self.y_max), UP));
        }
        for y in 0..self.y_max {
            starting_points.push((Point::new(-1, y), RIGHT));
            starting_points.push((Point::new(self.x_max, y), LEFT));
        }
        starting_points
            .iter()
            .map(|s| self.energized_from(*s))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: the test input needs to be adjusted to add an escape backslash before every backslash character
    const TEST_OFFICIAL_1: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 46);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 51);
    }
}
