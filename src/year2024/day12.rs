//! # 2024 day 12 - Garden Groups
//!
//! ## Part 1
//!
//! Floodfill with BFS to find the groups. For each point found during the search,
//! add +1 to the area, and +1 to the perimeter for each orthogonal neighbor
//! having a different value.
//!
//! ## Part 2
//!
//! Number of sides == number of corners. For each point found during the BFS,
//! then for each corner:
//! - If both orthogonal neighbors have the same value, then take a sneak peek on
//!   the diagonal point; if it has a different value, add +1 to corners.
//! - If both orthogonal neighbors have different value(s), add +1 to corners
//!   (either the diagonal also has a different value -> it's a corner; or it's
//!   the same value, which could mean either same or different group, and in
//!   both cases it means we've found a corner.)
//! - If only one of the orthogonal neighbors have the same value, we know
//!   it's not a corner.
use crate::aoc_util::{grid::*, point::*};
use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    garden_map: Grid,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            garden_map: Grid::parse(input),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut total_perimeter = 0;
        let mut total_sides = 0;
        let mut counted = HashSet::new();
        for y in 0..self.garden_map.y_max {
            for x in 0..self.garden_map.x_max {
                if !counted.contains(&(x, y)) {
                    let mut area = 0;
                    let mut perimeter = 0;
                    let mut sides = 0;
                    let mut group = HashSet::new();
                    let mut queue = VecDeque::from([Point::new(x as i32, y as i32)]);
                    let mut neighborstates = Vec::with_capacity(6);
                    while let Some(current) = queue.pop_front() {
                        if counted.contains(&(current.x as usize, current.y as usize)) {
                            continue;
                        }
                        counted.insert((current.x as usize, current.y as usize));
                        group.insert(current);
                        area += 1;
                        let element = self.garden_map.get_element(&current).unwrap();
                        for dir in NEIGHBORS_STRAIGHT.iter() {
                            let neighbor = current + *dir;
                            if let Some(neighbor_val) = self.garden_map.get_element(&neighbor) {
                                if neighbor_val == element {
                                    queue.push_back(neighbor);
                                    neighborstates.push((*dir, 1));
                                } else {
                                    perimeter += 1;
                                    neighborstates.push((*dir, 0));
                                }
                            } else {
                                perimeter += 1;
                                neighborstates.push((*dir, 0));
                            }
                        }
                        // Check the combinations of straight neighbors to evaluate corners
                        for (i, (n1, v1)) in neighborstates.iter().enumerate() {
                            for (n2, v2) in neighborstates.iter().skip(i) {
                                if (n1.x == 0 && n2.x == 0) || (n1.y == 0 && n2.y == 0) {
                                    continue;
                                }
                                if (*v1 == 0 && *v2 == 0)
                                    || (*v1 == 1
                                        && *v2 == 1
                                        && self
                                            .garden_map
                                            .get_element(&Point::new(
                                                current.x + n1.x + n2.x,
                                                current.y + n1.y + n2.y,
                                            ))
                                            .unwrap()
                                            != element)
                                {
                                    sides += 1;
                                }
                            }
                        }
                        neighborstates.clear();
                    }
                    total_perimeter += area * perimeter;
                    total_sides += area * sides;
                }
            }
        }
        (total_perimeter, total_sides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "AAAA
BBCD
BBCC
EEEC",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 140);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 772);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1930);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "AAAA
BBCD
BBCC
EEEC",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 80);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 236);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 368);
    }
}
