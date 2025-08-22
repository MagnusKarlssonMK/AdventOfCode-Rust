//! # 2023 day 10 - Pipe Maze
use std::{error::Error, str::FromStr};

use crate::aoc_util::{
    grid::Grid,
    point::{self, Point},
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

fn get_connection_directions(c: &char, outgoing: bool) -> Vec<Point> {
    match c {
        '|' => vec![point::UP, point::DOWN],
        '-' => vec![point::LEFT, point::RIGHT],
        'L' => {
            if outgoing {
                vec![point::UP, point::RIGHT]
            } else {
                vec![point::DOWN, point::LEFT]
            }
        }
        'J' => {
            if outgoing {
                vec![point::UP, point::LEFT]
            } else {
                vec![point::DOWN, point::RIGHT]
            }
        }
        '7' => {
            if outgoing {
                vec![point::DOWN, point::LEFT]
            } else {
                vec![point::UP, point::RIGHT]
            }
        }
        'F' => {
            if outgoing {
                vec![point::DOWN, point::RIGHT]
            } else {
                vec![point::UP, point::LEFT]
            }
        }
        _ => Vec::new(),
    }
}

struct InputData {
    grid: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        Ok(Self { grid })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let start_pos = self.grid.find('S').unwrap();
        let mut current_dir = point::UP;
        for d in &point::NEIGHBORS_STRAIGHT {
            if let Some(c) = self.grid.get_element(&(start_pos + *d))
                && get_connection_directions(&c, false).contains(d)
            {
                current_dir = *d;
                break;
            }
        }
        let mut pipe_path = vec![start_pos];
        let mut current_pos = start_pos + current_dir;
        while current_pos != start_pos {
            pipe_path.push(current_pos);
            for d in &get_connection_directions(&self.grid.get_element(&current_pos).unwrap(), true)
            {
                // Every point has two connections - make sure we don't go back the way we came in
                if !(d.x == -current_dir.x && d.y == -current_dir.y) {
                    current_dir = *d;
                    break;
                }
            }
            current_pos += current_dir;
        }
        let pipelen = pipe_path.len();

        // Calculate shoelace area
        // Add start point to the end of the path to connect also the last entry
        pipe_path.push(start_pos);
        let mut area = 0;
        for p in pipe_path.windows(2) {
            area += (p[0].x * p[1].y) - (p[1].x * p[0].y);
        }
        let area = (area.unsigned_abs() / 2) as usize;
        // Use Pick's theorem to calculate the contained area
        let p2 = area + 1 - (pipelen / 2);
        (pipelen / 2, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = ".....
.S-7.
.|.|.
.L-J.
.....";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 4);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 8);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 4);
    }

    #[test]
    fn part2_example_2() {
        let testdata = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 8);
    }
}
