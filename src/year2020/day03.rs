use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData<'a> {
    grid: Vec<&'a [u8]>
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self {
            grid: input.lines().map(|line| line.as_bytes()).collect()
        }
    }

    fn count_trees(&self, step: &Point) -> usize {
        let mut pos = ORIGIN;
        let mut count = 0;
        while pos.y < self.grid.len() as i32 {
            if self.grid[pos.y as usize][pos.x as usize] == b'#' {
                count += 1;
            }
            pos += *step;
            pos.x %= self.grid[0].len() as i32;
        }
        count
    }

    fn solve_part1(&self) -> usize {
        self.count_trees(&Point::new(3, 1))
    }

    fn solve_part2(&self) -> usize {
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)].iter()
            .map(|(r, c)| self.count_trees(&Point::new(*c, *r)))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 336);
    }
}
