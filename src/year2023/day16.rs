//! # 20xx day 16 - The Floor Will Be Lava
//!
//! Creates a map of connections between all splitters/mirrors to avoid having to walk step by step every time.
//! Slightly messy code, can probably be made prettier.
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

use crate::aoc_util::{grid::Grid, point::*};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

type NextPoint = Option<(Point, Vec<Point>)>;  // <Next point, new directions>

struct InputData {
    nodes: HashMap<Point, HashMap<Point, NextPoint>>, // From point: {out_direction: next_point}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let mut nodes = HashMap::new();
        // Find every point containing a bouncer or splitter, and create a map of their closest neighbor in each direction
        let mut points: Vec<Point> = grid
            .elements
            .iter()
            .enumerate()
            .filter(|(_, e)| **e != '.')
            .map(|(i, _)| grid.get_point(i))
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
            nodes.insert(*p, HashMap::new());
            // Find the closest neighbor in each direction that splits or bounces the light
            'neighbors: for d in &NEIGHBORS_STRAIGHT {
                let mut next = *p + *d;
                while let Some(n) = grid.get_element(&next) {
                    if bounces(&n, d) {
                        nodes.entry(*p).and_modify(|directionmap| {
                            directionmap.insert(*d, Some((next, get_next_directions(&n, d))));
                        });
                        continue 'neighbors;
                    }
                    next += *d;
                }
                // We reached the edge of the grid without hitting any bouncers
                // Take a step back
                next -= *d;
                if next != *p {
                    // We moved at least one step
                    nodes.entry(*p).and_modify(|directionmap| {
                        directionmap.insert(*d, Some((next, Vec::new())));
                    });
                } else {
                    // The node we started from is already on the edge in this direction
                    nodes.entry(*p).and_modify(|directionmap| {
                        directionmap.insert(*d, None);
                    });
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
    fn solve_part1(&self) -> usize {
        let mut energized: HashSet<Point> = HashSet::new();
        let mut seen: HashSet<(Point, Point)> = HashSet::new();
        let mut queue = VecDeque::from([(Point::new(-1, 0), RIGHT)]);

        while let Some((p, d)) = queue.pop_front() {
            if seen.contains(&(p, d)) {
                continue;
            }
            seen.insert((p, d));
            if let Some((next_p, next_d)) = self.nodes.get(&p).unwrap().get(&d).unwrap() {
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
        let mut best = 0;

        for s in &starting_points {
            let mut energized: HashSet<Point> = HashSet::new();
            let mut seen: HashSet<(Point, Point)> = HashSet::new();
            let mut queue = VecDeque::from([*s]);

            while let Some((p, d)) = queue.pop_front() {
                if seen.contains(&(p, d)) {
                    continue;
                }
                seen.insert((p, d));
                if let Some((next_p, next_d)) = self.nodes.get(&p).unwrap().get(&d).unwrap() {
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
            best = best.max(energized.len());
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: the test input needs to be adjusted to add an escape backslash before every backslash character
    const TEST_DATA: &str = ".|...\\....
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
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 46);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 51);
    }
}
