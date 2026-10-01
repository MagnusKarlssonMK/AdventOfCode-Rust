//! # 2024 day 6 - Guard Gallivant
//!
//! Starts with creating a jump table based on the grid, storing how far in each direction
//! the guard can go before either encountering an obstacle or falling out-of-bounds.
//!
//! Then the base path the guard takes is simulated by moving step-by-step and recording
//! each point+direction pair along the way, until the guard walks off the grid boundary.
//! The answer to part 1 can then be found by calculating the length of a set of points,
//! generated from the point-part of the tuples in the base path list.
//!
//! Then for part 2, iterate through each consecutive pair in the base path and put a
//! temporary obstacle in the second point, then use the jump table to step through the
//! grid and see if a loop is established. The number of loops found is the answer to part 2.
use crate::aoc_util::{grid::Grid, point::*};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    grid: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        let mut obstacles = HashSet::new();
        for (i, c) in grid.elements.iter().enumerate() {
            if *c == '#' {
                obstacles.insert(grid.get_point(i));
            }
        }
        Ok(Self { grid })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let guard_start_pos = self.grid.find('^').unwrap();

        // Simulate part 1 and record the base path
        let mut base_path: Vec<_> = Vec::new();
        let mut guard = guard_start_pos;
        let mut guard_dir = UP;
        while self.grid.get_element(&guard).is_some() {
            base_path.push((guard, guard_dir));
            let next_point = guard + guard_dir;
            if self.grid.get_element(&next_point) == Some('#') {
                guard_dir = guard_dir.rotate_right();
            } else {
                guard = next_point;
            }
        }
        let visited: HashSet<Point> = base_path.iter().map(|(p, _)| *p).collect();

        // Build the jump table
        let mut jump_table: Vec<HashMap<Point, (Point, bool)>> =
            self.grid.elements.iter().map(|_| HashMap::new()).collect();

        for d in NEIGHBORS_STRAIGHT {
            for (i, c) in self.grid.elements.iter().enumerate() {
                if *c == '#' {
                    continue;
                }
                let mut p = self.grid.get_point(i);
                loop {
                    let np = p + d;
                    if let Some(nc) = self.grid.get_element(&np) {
                        if nc == '#' {
                            jump_table[i].insert(d, (p, false));
                            break;
                        }
                    } else {
                        jump_table[i].insert(d, (p, true));
                        break;
                    }
                    p = np;
                }
            }
        }

        let mut p2 = 0;
        let mut obstacles_tried = HashSet::new();
        for w in base_path.windows(2) {
            let (mut cp, mut cd) = w[0];
            let (new_obstacle, _) = w[1];
            if cp == new_obstacle || obstacles_tried.contains(&new_obstacle) {
                continue;
            }
            obstacles_tried.insert(new_obstacle);

            let mut seen = HashSet::new();
            loop {
                if seen.contains(&(cp, cd)) {
                    p2 += 1;
                    break;
                }
                seen.insert((cp, cd));
                let (np, oob) = jump_table[self.grid.get_index(&cp)].get(&cd).unwrap();
                let sign = if cd.x + cd.y > 0 { 1 } else { -1 };
                let blocked = (cd.x == 0
                    && cp.x == new_obstacle.x
                    && 0 < sign * (new_obstacle.y - cp.y)
                    && sign * (new_obstacle.y - cp.y) <= sign * (np.y - cp.y))
                    || (cd.y == 0
                        && cp.y == new_obstacle.y
                        && 0 < sign * (new_obstacle.x - cp.x)
                        && sign * (new_obstacle.x - cp.x) <= sign * (np.x - cp.x));
                if blocked {
                    cp = Point {
                        x: new_obstacle.x - cd.x,
                        y: new_obstacle.y - cd.y,
                    };
                    cd = cd.rotate_right();
                } else if *oob {
                    break;
                } else {
                    cp = *np;
                    cd = cd.rotate_right();
                }
            }
        }

        (visited.len(), p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 41);
        assert_eq!(p2, 6);
    }
}
