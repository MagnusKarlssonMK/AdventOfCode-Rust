use std::cmp::{max, min};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Operation {
    fn from(line: &str) -> Self {
        match line.as_bytes().get(6).copied() {
            Some(b'n') => Self::TurnOn,
            Some(b'f') => Self::TurnOff,
            Some(b' ') => Self::Toggle,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Area {
    x_range: (usize, usize),
    y_range: (usize, usize),
}

impl Area {
    fn from(line: &str) -> Self {
        let nbrs: Vec<usize> = line
            .split([' ', ','])
            .filter_map(|s| s.parse().ok())
            .collect();
        Self {
            x_range: (min(nbrs[0], nbrs[2]), max(nbrs[0], nbrs[2])),
            y_range: (min(nbrs[1], nbrs[3]), max(nbrs[1], nbrs[3])),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    area: Area,
}

impl Instruction {
    fn from(line: &str) -> Self {
        Self {
            op: Operation::from(line),
            area: Area::from(line),
        }
    }
}

const GRIDSIZE: usize = 1000;

struct InputData {
    santa_instructions: Vec<Instruction>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            santa_instructions: input.lines().map(Instruction::from).collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        let mut grid = vec![vec![false; GRIDSIZE]; GRIDSIZE];
        for instr in self.santa_instructions.iter() {
            for x in instr.area.x_range.0..=instr.area.x_range.1 {
                for g_y in grid
                    .iter_mut()
                    .take(instr.area.y_range.1 + 1)
                    .skip(instr.area.y_range.0)
                {
                    g_y[x] = match instr.op {
                        Operation::TurnOff => false,
                        Operation::TurnOn => true,
                        Operation::Toggle => !g_y[x],
                    }
                }
            }
        }
        grid.iter()
            .map(|y| y.iter().filter(|&&state| state).count())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut grid = vec![vec![0; GRIDSIZE]; GRIDSIZE];
        for instr in self.santa_instructions.iter() {
            for x in instr.area.x_range.0..=instr.area.x_range.1 {
                for g_y in grid
                    .iter_mut()
                    .take(instr.area.y_range.1 + 1)
                    .skip(instr.area.y_range.0)
                {
                    g_y[x] = match instr.op {
                        Operation::TurnOff => {
                            if g_y[x] > 0 {
                                g_y[x] - 1
                            } else {
                                0
                            }
                        }
                        Operation::TurnOn => g_y[x] + 1,
                        Operation::Toggle => g_y[x] + 2,
                    }
                }
            }
        }
        grid.iter().map(|y| y.iter().sum::<usize>()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("turn on 0,0 through 999,999");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1000 * 1000);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("toggle 0,0 through 999,0");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1000);
    }

    #[test]
    fn part1_example_3() {
        let testdata =
            String::from("turn on 0,0 through 999,999\nturn off 499,499 through 500,500");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1000 * 1000 - 4);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("turn on 0,0 through 0,0");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("toggle 0,0 through 999,999");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2000000);
    }
}
