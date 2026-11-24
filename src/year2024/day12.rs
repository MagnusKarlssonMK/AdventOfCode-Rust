//! # 2024 day 12 - Garden Groups
//!
//! ## Part 1
//!
//! Floodfill with BFS to find the groups. For each point found during the search, add +1 to the
//! area, and +1 to the perimeter for each orthogonal neighbor having a different value.
//!
//! ## Part 2
//!
//! Number of sides == number of corners. For each point found during the BFS, then for each
//! corner:
//! * If both orthogonal neighbors have the same value, then take a sneak peek on the diagonal
//!   point; if it has a different value, add +1 to corners.
//! * If both orthogonal neighbors have different value(s), add +1 to corners (either the diagonal
//!   also has a different value -> it's a corner; or it's the same value, which could mean either
//!   same or different group, and in both cases it means we've found a corner.)
//! * If only one of the orthogonal neighbors have the same value, we know it's not a corner.
use crate::aoc_util::{error::AocError, grid::*, point::*};
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData {
    garden_map: Grid<char>,
}

impl FromStr for InputData {
    type Err = AocError;
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
        for (p, v) in self.garden_map.enumerate_points() {
            if !counted.contains(&p) {
                let mut area = 0;
                let mut perimeter = 0;
                let mut sides = 0;
                let mut queue = VecDeque::from([(p, *v)]);
                let mut neighborstates = Vec::with_capacity(6);
                while let Some((current_point, current_val)) = queue.pop_front() {
                    if counted.contains(&current_point) {
                        continue;
                    }
                    counted.insert(current_point);
                    area += 1;
                    for dir in NEIGHBORS_STRAIGHT.iter() {
                        let neighbor = current_point + *dir;
                        if let Some(neighbor_val) = self.garden_map.get_element(&neighbor) {
                            if neighbor_val == current_val {
                                queue.push_back((neighbor, neighbor_val));
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
                                        .get_ref(&Point::new(
                                            current_point.x + n1.x + n2.x,
                                            current_point.y + n1.y + n2.y,
                                        ))
                                        .is_some_and(|&diag| diag != current_val))
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
        (total_perimeter, total_sides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const TEST_OFFICIAL_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_OFFICIAL_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_OFFICIAL_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST_OFFICIAL_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 140);
        assert_eq!(p2, 80);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 772);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1930);
    }

    #[test]
    fn part2_official_4() {
        let solution_data: InputData = TEST_OFFICIAL_4.parse().unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 236);
    }

    #[test]
    fn part2_official_5() {
        let solution_data: InputData = TEST_OFFICIAL_5.parse().unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 368);
    }
}
