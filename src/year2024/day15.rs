//! # 2024 day 15 - Warehouse Woes
//!
//! ## Part 1
//!
//! Uses recursion to investigate each move, and performs the move immediately if found
//! to be possible.
//!
//! ## Part 2
//!
//! After expanding the map, horizontal moves are still performed as in part 1. However
//! for vertical moves, we first need to completely check if the move is possible before
//! carrying it out, since it can now branch out sideways. This check is also done recursively,
//! storing the nodes investigated in a hashmap. If the move passes the check, the hashmap
//! is used to swap those nodes in the direction of the move, by going through the elements
//! and finding the ones with empty space above and swapping. The number of nodes involved
//! is mostly quite low, so we can get away with such a simplistic approach.
use crate::aoc_util::{grid::*, point::*};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

impl Grid {
    #[inline]
    fn make_wide(&self) -> Self {
        let x_max = 2 * self.x_max;
        let y_max = 2 * self.y_max;
        let mut elements = Vec::with_capacity(2 * x_max * y_max);
        self.elements.iter().for_each(|c| match c {
            '@' => {
                elements.push(*c);
                elements.push('.');
            }
            'O' => {
                elements.push('[');
                elements.push(']');
            }
            _ => {
                elements.push(*c);
                elements.push(*c);
            }
        });
        Self {
            x_max,
            y_max,
            elements,
        }
    }

    fn move_simple(&mut self, pos: &Point, dir: &Point) -> bool {
        let nextpos = *pos + *dir;
        match self.get_element(&nextpos) {
            Some('#') | None => false,
            Some('O') | Some('[') | Some(']') => {
                if self.move_simple(&nextpos, dir) {
                    let i1 = self.get_index(pos);
                    let i2 = self.get_index(&nextpos);
                    self.elements.swap(i1, i2);
                    true
                } else {
                    false
                }
            }
            _ => {
                let i1 = self.get_index(pos);
                let i2 = self.get_index(&nextpos);
                self.elements.swap(i1, i2);
                true
            }
        }
    }

    fn move_double(&mut self, pos: &HashSet<Point>, dir: &Point) {
        let mut queue = VecDeque::from_iter(pos.iter());
        while !queue.is_empty() {
            // Move the elements that have empty space on top, keep rotating the queue
            // until one of those nodes are in the first position.
            let from_point = *queue.front().unwrap();
            let to_point = *from_point + *dir;
            if self.get_element(&to_point).unwrap() == '.' {
                queue.pop_front();
                let from = self.get_index(from_point);
                let to = self.get_index(&to_point);
                self.elements.swap(from, to);
            } else {
                queue.rotate_left(1);
            }
        }
    }

    fn check_double(&self, pos: &Point, dir: &Point, checked: &mut HashSet<Point>) -> bool {
        checked.insert(*pos);
        let nextpos = *pos + *dir;
        match self.get_element(&nextpos) {
            Some('#') | None => false,
            Some('[') => {
                let rightpos = nextpos + RIGHT;
                self.check_double(&nextpos, dir, checked)
                    && (checked.contains(&rightpos) || self.check_double(&rightpos, dir, checked))
            }
            Some(']') => {
                let leftpos = nextpos + LEFT;
                self.check_double(&nextpos, dir, checked)
                    && (checked.contains(&leftpos) || self.check_double(&leftpos, dir, checked))
            }
            _ => true,
        }
    }
}

impl Point {
    #[inline]
    fn from_dir(input: char) -> Option<Self> {
        match input {
            '>' => Some(RIGHT),
            '^' => Some(UP),
            'v' => Some(DOWN),
            '<' => Some(LEFT),
            _ => None, // There are line breaks in the input!
        }
    }
}

struct InputData {
    grid: Grid,
    robot: Point,
    moves: Vec<Point>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (boxstr, movestr) = s.split_once("\n\n").unwrap();
        let grid = Grid::parse(boxstr);
        let robot = grid.find('@').unwrap();
        Ok(Self {
            grid,
            robot,
            moves: movestr.chars().filter_map(Point::from_dir).collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut bumped_grid = self.grid.clone();
        let mut robot_pos = self.robot;
        for mv_dir in self.moves.iter() {
            if bumped_grid.move_simple(&robot_pos, mv_dir) {
                robot_pos += *mv_dir;
            }
        }
        bumped_grid
            .elements
            .iter()
            .enumerate()
            .filter(|(_, e)| **e == 'O')
            .map(|(i, _)| 100 * (i / bumped_grid.x_max) + i % bumped_grid.x_max)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut bumped_grid = self.grid.make_wide();
        let mut robot_pos = Point::new(2 * self.robot.x, self.robot.y);
        for mv_dir in self.moves.iter() {
            if mv_dir.y == 0 {
                if bumped_grid.move_simple(&robot_pos, mv_dir) {
                    robot_pos += *mv_dir;
                }
            } else {
                let mut checked: HashSet<Point> = HashSet::new();
                // First need to do a recursive check to see if the move is possible
                // before actually moving anything with a recursive move
                if bumped_grid.check_double(&robot_pos, mv_dir, &mut checked) {
                    bumped_grid.move_double(&checked, mv_dir);
                    robot_pos += *mv_dir;
                }
            }
        }
        bumped_grid
            .elements
            .iter()
            .enumerate()
            .filter(|(_, e)| **e == '[')
            .map(|(i, _)| 100 * (i / bumped_grid.x_max) + i % bumped_grid.x_max)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const TEST_DATA_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    const TEST_DATA_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 2028);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part1(), 10092);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_3).unwrap();
        assert_eq!(solution_data.solve_part2(), 105 + 207 + 306);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::from_str(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 9021);
    }
}
