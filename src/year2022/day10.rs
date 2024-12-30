//! # 2022 day 10 - Cathode-Ray Tube
use crate::aoc_util::{grid::*, point::*};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

enum Opcode {
    Noop,
    Addx(isize),
}

impl Opcode {
    fn new(input: &str) -> Self {
        if input.starts_with('n') {
            Self::Noop
        } else {
            let (_, v) = input.split_once(' ').unwrap();
            Self::Addx(v.parse().unwrap())
        }
    }
}

struct InputData {
    program: Vec<Opcode>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            program: input.lines().map(Opcode::new).collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        const INTERVALS: [usize; 6] = [20, 60, 100, 140, 180, 220];
        let mut total: isize = 0;
        let mut reg_x: isize = 1;
        let mut cyclenbr: usize = 0;
        let mut tick = |x| {
            cyclenbr += 1;
            if INTERVALS.contains(&cyclenbr) {
                total += x * cyclenbr as isize;
            }
        };
        for p in &self.program {
            match p {
                Opcode::Noop => {
                    tick(reg_x);
                }
                Opcode::Addx(v) => {
                    for _ in 0..2 {
                        tick(reg_x);
                    }
                    reg_x += *v;
                }
            }
        }
        total as usize
    }

    fn solve_part2(&self) -> String {
        let mut crt = Grid::new(40, 6, ' ');
        let mut reg_x: isize = 1;
        let mut cyclenbr: usize = 0;
        let mut tick = |x: isize| {
            if (x - (cyclenbr % crt.x_max) as isize).abs() <= 1 {
                crt.set_point(
                    &Point::new((cyclenbr % 40) as i32, (cyclenbr / 40) as i32),
                    '#',
                );
            }
            cyclenbr += 1;
        };
        for p in &self.program {
            match p {
                Opcode::Noop => {
                    tick(reg_x);
                }
                Opcode::Addx(v) => {
                    for _ in 0..2 {
                        tick(reg_x);
                    }
                    reg_x += *v;
                }
            }
        }
        let mut result = Vec::with_capacity(crt.elements.len() + crt.y_max + 1);
        for y in 0..crt.y_max {
            result.push('\n');
            for x in 0..crt.x_max {
                result.push(crt.get_element(&Point::new(x as i32, y as i32)).unwrap());
            }
        }
        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "addx 15
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
noop",
        );
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 13140);
        assert_eq!(
            solution_data.solve_part2(),
            "\n##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     "
        );
    }
}
