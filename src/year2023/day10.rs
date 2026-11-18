//! # 2023 day 10 - Pipe Maze
//!
//! Store the data in a grid, then for Part 1 simply walk through the system until returning to the
//! start, then calculating the answer by dividing the number of steps taken by 2. For part 2,
//! calculate the answer by first determining the area with the shoelace formula and then use that
//! with Pick's theorem.
use crate::aoc_util::{error::AocError, grid::Grid, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

impl Point {
    fn area(&self, other: &Point) -> i32 {
        (self.x * other.y) - (other.x * self.y)
    }
}

/// Calculates the next direction when moving into a point with element `c` in direction `in_dir`.
/// Returns `None` if `c` is an invalid value or if there is no connection into that value with
/// incoming direction `in_dir`.
fn next_direction(c: &char, in_dir: &Point) -> Option<Point> {
    if *c == 'S' {
        // We reached the starting point again; return any direction, the loop will end regardless
        // of value
        Some(*in_dir)
    } else {
        match *in_dir {
            UP => match c {
                '|' => Some(UP),
                '7' => Some(LEFT),
                'F' => Some(RIGHT),
                _ => None,
            },
            DOWN => match c {
                '|' => Some(DOWN),
                'J' => Some(LEFT),
                'L' => Some(RIGHT),
                _ => None,
            },
            LEFT => match c {
                '-' => Some(LEFT),
                'L' => Some(UP),
                'F' => Some(DOWN),
                _ => None,
            },
            RIGHT => match c {
                '-' => Some(RIGHT),
                'J' => Some(UP),
                '7' => Some(DOWN),
                _ => None,
            },
            _ => None,
        }
    }
}

struct InputData {
    grid: Grid<char>,
    start: Point,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse(s);
        if let Some(start) = grid.find(&'S') {
            Ok(Self { grid, start })
        } else {
            Err(AocError::Invalid("start point not found".to_string()))
        }
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut current_dir = NEIGHBORS_STRAIGHT
            .into_iter()
            .find(|d| {
                self.grid
                    .get_ref(&(self.start + *d))
                    .is_some_and(|c| next_direction(c, d).is_some())
            })
            .ok_or(AocError::NoSolution)?;
        let mut current_pos = self.start + current_dir;
        let mut pipe_len = 1;
        let mut shoelace_area = current_pos.area(&self.start);
        while current_pos != self.start {
            if let Some(c) = self.grid.get_ref(&current_pos) {
                if let Some(new_dir) = next_direction(c, &current_dir) {
                    current_dir = new_dir;
                    let new_pos = current_pos + current_dir;
                    pipe_len += 1;
                    shoelace_area += new_pos.area(&current_pos);
                    current_pos = new_pos;
                } else {
                    // No connection to next node
                    return Err(AocError::NoSolution);
                }
            } else {
                // Path leading out-of-bounds
                return Err(AocError::NoSolution);
            }
        }

        let shoelace_area: usize = (shoelace_area.unsigned_abs() / 2) as usize;
        // Use Pick's theorem to calculate the contained area
        let p2 = shoelace_area + 1 - (pipe_len / 2);
        Ok((pipe_len / 2, p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_OFFICIAL_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const TEST_OFFICIAL_3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const TEST_OFFICIAL_4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, _) = solution_data.solve().unwrap();
        assert_eq!(p1, 4);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (p1, _) = solution_data.solve().unwrap();
        assert_eq!(p1, 8);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        let (_, p2) = solution_data.solve().unwrap();
        assert_eq!(p2, 4);
    }

    #[test]
    fn part2_official_4() {
        let solution_data: InputData = TEST_OFFICIAL_4.parse().unwrap();
        let (_, p2) = solution_data.solve().unwrap();
        assert_eq!(p2, 8);
    }

    #[test]
    fn parser_no_start() {
        assert!(matches!(
            "-7F\n|.J\n--L".parse::<InputData>(),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn broken_paths() {
        for input in [
            "---\n|S|\n---", // No way out of S
            "-7|\n|SF\n-FL", // Path leading out of grid
            "F7.\n|S7\n--J", // Bad connection
        ] {
            assert!(matches!(
                input.parse::<InputData>().unwrap().solve(),
                Err(AocError::NoSolution)
            ));
        }
    }
}
