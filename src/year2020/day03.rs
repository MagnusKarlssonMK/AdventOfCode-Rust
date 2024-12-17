use crate::aoc_util::{grid::*, point::*};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    grid: Grid,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            grid: Grid::parse(input),
        }
    }

    fn count_trees(&self, step: &Point) -> usize {
        let mut pos = ORIGIN;
        let mut count = 0;
        while let Some(e) = self.grid.get_element(&pos) {
            if e == '#' {
                count += 1;
            }
            pos += *step;
            pos.x %= self.grid.x_max as i32;
        }
        count
    }

    fn solve_part1(&self) -> usize {
        self.count_trees(&Point::new(3, 1))
    }

    fn solve_part2(&self) -> usize {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(x, y)| self.count_trees(&Point::new(*x, *y)))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 336);
    }
}
