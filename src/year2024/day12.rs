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
//! * If both orthogonal neighbors have the same value, then take a sneak peek on
//!   the diagonal point; if it has a different value, add +1 to corners.
//! * If both orthogonal neighbors have different value(s), add +1 to corners
//!   (either the diagonal also has a different value -> it's a corner; or it's
//!   the same value, which could mean either same or different group, and in
//!   both cases it means we've found a corner.)
//! * If only one of the orthogonal neighbors have the same value, we know
//!   it's not a corner.
use crate::aoc_util::{grid::*, point::*};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    garden_map: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            garden_map: Grid::parse(s),
        })
    }
}

impl InputData {
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

    const TEST_DATA_1: &str = "AAAA
BBCD
BBCC
EEEC";
    const TEST_DATA_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const TEST_DATA_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const TEST_DATA_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    const TEST_DATA_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 140);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 772);
    }

    #[test]
    fn part1_example_3() {
        let solution_data = InputData::from_str(TEST_DATA_3).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1930);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 80);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_4).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 236);
    }

    #[test]
    fn part2_example_3() {
        let solution_data = InputData::from_str(TEST_DATA_5).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 368);
    }
}
