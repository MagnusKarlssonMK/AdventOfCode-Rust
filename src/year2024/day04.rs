//! # 2024 day 4 - Ceres Search
//!
//! ## Part 1
//! 
//! Scan the grid and investigate all nodes containing an 'X', and look in all
//! eight directions and see if XMAS is created.
//!
//! ## Part 2
//!
//! This time, scan for nodes containing an 'A' instead, form words from the
//! combination with the diagonal nodes to form the X, and see if the generated
//! word is eithes MAS or SAM.
pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    width: isize,
    height: isize,
    grid: Vec<Vec<char>>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self {
            width: grid[0].len() as isize,
            height: grid.len() as isize,
            grid,
        }
    }

    fn get_value(&self, row: isize, col: isize) -> char {
        if (0..self.height).contains(&row) && (0..self.width).contains(&col) {
            self.grid[row as usize][col as usize]
        } else {
            '.'
        }
    }

    fn solve_part1(&self) -> usize {
        let mut total = 0;
        const DIRECTIONS: [(isize, isize); 8] = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get_value(row, col) == 'X' {
                    for (d_row, d_col) in DIRECTIONS {
                        let word: String = [
                            'X',
                            self.get_value(row + d_row, col + d_col),
                            self.get_value(row + 2 * d_row, col + 2 * d_col),
                            self.get_value(row + 3 * d_row, col + 3 * d_col),
                        ]
                        .iter()
                        .collect();
                        if word == "XMAS" {
                            total += 1;
                        }
                    }
                }
            }
        }
        total
    }

    fn solve_part2(&self) -> usize {
        let mut total = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get_value(row, col) == 'A' {
                    let word1: String = [
                        self.get_value(row - 1, col - 1),
                        'A',
                        self.get_value(row + 1, col + 1),
                    ]
                    .iter()
                    .collect();
                    if word1 == "MAS" || word1 == "SAM" {
                        let word2: String = [
                            self.get_value(row + 1, col - 1),
                            'A',
                            self.get_value(row - 1, col + 1),
                        ]
                        .iter()
                        .collect();
                        if word2 == "MAS" || word2 == "SAM" {
                            total += 1;
                        }
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 9);
    }
}
