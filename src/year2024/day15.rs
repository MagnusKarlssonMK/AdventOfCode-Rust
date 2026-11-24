//! # 2024 day 15 - Warehouse Woes
//!
//! ## Part 1
//!
//! Uses recursion to investigate each move, and performs the move immediately if found to be
//! possible.
//!
//! ## Part 2
//!
//! After expanding the map, horizontal moves are still performed as in part 1. However for
//! vertical moves, we first need to completely check if the move is possible before carrying it
//! out, since it can now branch out sideways. This check is also done recursively, storing the
//! nodes investigated in a hashmap. If the move passes the check, the hashmap is used to swap
//! those nodes in the direction of the move, by going through the elements and finding the ones
//! with empty space above and swapping. The number of nodes involved is mostly quite low, so we
//! can get away with such a simplistic approach.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    grid::*,
    point::*,
};
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

impl Grid<char> {
    #[inline]
    fn make_wide(&self) -> Self {
        let x_max = 2 * self.x_max;
        let y_max = self.y_max;
        let mut elements = Vec::with_capacity(x_max * y_max);
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

    #[inline]
    fn swap_points(&mut self, p1: &Point, p2: &Point) {
        let i1 = self.get_index(p1);
        let i2 = self.get_index(p2);
        self.elements.swap(i1, i2);
    }

    fn move_simple(&mut self, pos: &Point, dir: &Point) -> bool {
        let nextpos = *pos + *dir;
        match self.get_element(&nextpos) {
            Some('#') | None => false,
            Some('O') | Some('[') | Some(']') => {
                if self.move_simple(&nextpos, dir) {
                    self.swap_points(pos, &nextpos);
                    true
                } else {
                    false
                }
            }
            _ => {
                self.swap_points(pos, &nextpos);
                true
            }
        }
    }

    fn move_double(&mut self, pos: &HashSet<Point>, dir: &Point) {
        let mut queue = VecDeque::from_iter(pos.iter());
        while let Some(&from_point) = queue.front() {
            // Move the elements that have empty space on top, keep rotating the queue
            // until one of those nodes are in the first position.
            let to_point = *from_point + *dir;
            if let Some(v) = self.get_ref(&to_point)
                && *v == '.'
            {
                queue.pop_front();
                self.swap_points(from_point, &to_point);
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
    fn from_dir(input: char) -> Result<Self, AocError> {
        match input {
            '>' => Ok(RIGHT),
            '^' => Ok(UP),
            'v' => Ok(DOWN),
            '<' => Ok(LEFT),
            _ => Err(AocError::Invalid(format!("invalid direction: {input}"))),
        }
    }
}

struct InputData {
    grid: Grid<char>,
    robot: Point,
    moves: Vec<Point>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (boxstr, movestr) = s.split_once("\n\n").ctx("missing block separator")?;
        let grid = Grid::parse(boxstr);
        let robot = grid.find(&'@').ctx("robot missing in grid")?;
        Ok(Self {
            grid,
            robot,
            moves: movestr
                .chars()
                .filter(|&c| c != '\n') // There are line breaks in the input
                .map(Point::from_dir)
                .collect::<Result<_, _>>()?,
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
            .enumerate_points()
            .filter(|&(_, e)| *e == 'O')
            .map(|(p, _)| 100 * p.y as usize + p.x as usize)
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
            .enumerate_points()
            .filter(|&(_, e)| *e == '[')
            .map(|(p, _)| 100 * p.y as usize + p.x as usize)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const TEST_OFFICIAL_2: &str = "##########
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

    const TEST_OFFICIAL_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 2028);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 10092);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 9021);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 105 + 207 + 306);
    }
}
