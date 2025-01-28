//! # 2020 day 11 - Seating System
//!
//! For each cycle, count the number of visible neighbors for each point
//! by scanning the room in all eight directions, then generate the next
//! iteration of the room based on that.
//!
//! The scan loops have been combined as much as possible both to shrink the
//! code and to minimize the number of loops, at the cost of making it a
//! bit harder to understand the logic from reading the code.
use crate::aoc_util::{grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData {
    grid: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::parse(s),
        })
    }
}

impl InputData {
    fn get_steady_state(&self, only_adjacent: bool) -> usize {
        let mut current = self.grid.clone();
        let mut counts: Vec<u8> = vec![0; current.elements.len()];
        let neighbor_limit = if only_adjacent { 4 } else { 5 };
        loop {
            let mut scan = |start: &Point, dir: &Point| {
                let mut prev = 'L';
                let mut current_point = *start;
                while let Some(c) = current.get_element(&current_point) {
                    if prev == '#' {
                        counts[current.get_index(&current_point)] += 1;
                    }
                    if c != '.' || only_adjacent {
                        prev = c;
                    }
                    current_point += *dir;
                }
            };
            for x in 0..current.x_max {
                scan(&Point::new(x as i32, 0), &DOWN);
                scan(&Point::new(x as i32, (current.y_max - 1) as i32), &UP);
                if x != 0 {
                    scan(&Point::new(x as i32, (current.y_max - 1) as i32), &DIAG_R_U);
                    scan(&Point::new(x as i32, 0), &DIAG_R_D);
                }
                if x != current.x_max - 1 {
                    scan(&Point::new(x as i32, (current.y_max - 1) as i32), &DIAG_L_U);
                    scan(&Point::new(x as i32, 0), &DIAG_L_D);
                }
            }
            for y in 0..current.y_max {
                scan(&Point::new(0, y as i32), &RIGHT);
                scan(&Point::new((current.x_max - 1) as i32, y as i32), &LEFT);
                scan(&Point::new(0, y as i32), &DIAG_R_U);
                scan(&Point::new(0, y as i32), &DIAG_R_D);
                scan(&Point::new((current.x_max - 1) as i32, y as i32), &DIAG_L_U);
                scan(&Point::new((current.x_max - 1) as i32, y as i32), &DIAG_L_D);
            }

            let next: Vec<_> = counts
                .iter()
                .enumerate()
                .map(|(i, v)| match current.elements[i] {
                    '#' => {
                        if *v >= neighbor_limit {
                            'L'
                        } else {
                            '#'
                        }
                    }
                    'L' => {
                        if *v == 0 {
                            '#'
                        } else {
                            'L'
                        }
                    }
                    _ => '.',
                })
                .collect();
            if next == current.elements {
                return next.iter().filter(|c| **c == '#').count();
            }
            current.elements = next;
            for v in &mut counts {
                *v = 0
            }
        }
    }

    fn solve_part1(&self) -> usize {
        self.get_steady_state(true)
    }

    fn solve_part2(&self) -> usize {
        self.get_steady_state(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 37);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 26);
    }
}
