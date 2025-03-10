//! # 2024 day 13 - Claw Contraption
//!
//! Calculates the answer basically by solving a matrix. The same function can be
//! used for both parts, with an argument to input the extra scaling value for part 2.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl FromStr for ClawMachine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let b: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let p: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        Ok(Self {
            button_a: (
                a[2].strip_suffix(',')
                    .unwrap()
                    .split_once('+')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                a[3].split_once('+').unwrap().1.parse().unwrap(),
            ),
            button_b: (
                b[2].strip_suffix(',')
                    .unwrap()
                    .split_once('+')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                b[3].split_once('+').unwrap().1.parse().unwrap(),
            ),
            prize: (
                p[1].strip_suffix(',')
                    .unwrap()
                    .split_once('=')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                p[2].split_once('=').unwrap().1.parse().unwrap(),
            ),
        })
    }
}

impl ClawMachine {
    fn get_win_tokens(&self, extra: usize) -> Option<usize> {
        let p_x = self.prize.0 + extra as isize;
        let p_y = self.prize.1 + extra as isize;
        let i = (self.button_b.0 * p_y - self.button_b.1 * p_x)
            / (self.button_a.1 * self.button_b.0 - self.button_a.0 * self.button_b.1);
        let j = (p_x - i * self.button_a.0) / self.button_b.0;
        if (p_x - i * self.button_a.0) % self.button_b.0 == 0
            && (self.button_b.0 * p_y - self.button_b.1 * p_x)
                % (self.button_a.1 * self.button_b.0 - self.button_a.0 * self.button_b.1)
                == 0
        {
            Some((3 * i + j) as usize)
        } else {
            None
        }
    }
}

struct InputData {
    machines: Vec<ClawMachine>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            machines: s
                .split("\n\n")
                .map(|m| ClawMachine::from_str(m).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.machines
            .iter()
            .filter_map(|m| m.get_win_tokens(0))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.machines
            .iter()
            .filter_map(|m| m.get_win_tokens(10000000000000))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 480);
    }
}
