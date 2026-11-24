//! # 2022 day 10 - Cathode-Ray Tube
//!
//! Both parts replay the same program one tick at a time, with the `X` register starting at 1.
//! `noop` takes a single tick, while `addx` takes two ticks before changing the register. `run`
//! owns that loop and calls a closure for every tick, so part 1 only sums signal strength at the
//! six checkpoints, and part 2 only draws a pixel when it falls inside the sprite.
use crate::aoc_util::{error::AocError, grid::*, point::*};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2(),
    ))
}

const P2_CRT_WIDTH: i32 = 40;
const P2_CRT_HEIGHT: i32 = 6;

enum Opcode {
    Noop,
    Addx(i32),
}

impl FromStr for Opcode {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None if s == "noop" => Ok(Self::Noop),
            Some(("addx", v)) => Ok(Self::Addx(v.parse()?)),
            _ => Err(AocError::Invalid(format!("invalid opcode: {s}"))),
        }
    }
}

struct InputData {
    program: Vec<Opcode>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            program: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn run(&self, mut tick: impl FnMut(i32)) {
        let mut reg_x = 1;
        for op in &self.program {
            match op {
                Opcode::Noop => tick(reg_x),
                Opcode::Addx(v) => {
                    tick(reg_x);
                    tick(reg_x);
                    reg_x += v;
                }
            }
        }
    }

    fn solve_part1(&self) -> i32 {
        const INTERVALS: [i32; 6] = [20, 60, 100, 140, 180, 220];
        let mut total = 0;
        let mut cycle = 0;
        self.run(|reg_x| {
            cycle += 1;
            if INTERVALS.contains(&cycle) {
                total += reg_x * cycle;
            }
        });
        total
    }

    fn solve_part2(&self) -> String {
        let mut crt = Grid::new(P2_CRT_WIDTH as usize, P2_CRT_HEIGHT as usize, &'.');
        let mut cycle = 0;
        self.run(|reg_x| {
            let x = cycle % P2_CRT_WIDTH;
            let y = cycle / P2_CRT_WIDTH;
            if (reg_x - x).abs() <= 1 {
                crt.set_point(&Point::new(x, y), '#');
            }
            cycle += 1;
        });
        format!("\n{}", crt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const OFFICIAL_1_P2_RESULT: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 13140);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), OFFICIAL_1_P2_RESULT);
    }

    #[test]
    fn parser_invalid_input() {
        assert!("bogus".parse::<InputData>().is_err());
        assert!("addx x".parse::<InputData>().is_err());
    }
}
