//! # 2023 day 18 - Lavaduct Lagoon
//!
//! Similar to day 10; the path is generated differently, but the solution eventually also comes
//! down to using the shoelace formula into Pick's theorem to calculate the area. The main
//! difference is that unlike in day 10, here the area should *include* the boundary, so we need to
//! add half the dig length rather than subtract it in the final calculation.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    point::*,
};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Instruction {
    p1_step: Point,
    p2_step: Point,
}

impl FromStr for Instruction {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let p1_direction = match parts.next().ctx("missing direction")? {
            "L" => LEFT,
            "R" => RIGHT,
            "U" => UP,
            "D" => DOWN,
            _ => {
                return Err(AocError::Invalid(format!("invalid direction: {s}")));
            }
        };
        let p1_steps: u32 = parts.next().ctx("missing steps")?.parse()?;
        let color = parts
            .next()
            .ctx("missing color")?
            .trim_start_matches("(#")
            .trim_end_matches(')')
            .to_string();
        if color.len() != 6 {
            return Err(AocError::Invalid(format!("invalid color: {s}")));
        }
        let p2_direction = match color.bytes().last() {
            Some(b'0') => RIGHT,
            Some(b'1') => DOWN,
            Some(b'2') => LEFT,
            Some(b'3') => UP,
            _ => {
                return Err(AocError::Invalid(format!("invalid color: {s}")));
            }
        };
        // Note: we've already verified that there's 6 characters in color, so the indexing is safe
        let p2_steps = u32::from_str_radix(&color[0..5], 16)?;
        Ok(Self {
            p1_step: p1_direction * p1_steps as i32,
            p2_step: p2_direction * p2_steps as i32,
        })
    }
}

struct InputData {
    plan: Vec<Instruction>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            plan: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn area_points(&self, swapped: bool) -> usize {
        let mut dig_len = 0;
        let mut shoelace_area = 0;
        let mut current_point = ORIGIN;
        for instr in &self.plan {
            let next_point = if !swapped {
                current_point + instr.p1_step
            } else {
                current_point + instr.p2_step
            };
            dig_len += next_point.manhattan(&current_point);
            shoelace_area += next_point.cross(&current_point);
            current_point = next_point
        }
        if current_point != ORIGIN {
            // Closing the loop just in case.
            // Real input guarantees that we end up where we started.
            dig_len += current_point.manhattan(&ORIGIN);
        }

        let areasum = (shoelace_area.unsigned_abs() / 2) as usize;
        areasum + 1 + (dig_len / 2)
    }

    fn solve_part1(&self) -> usize {
        self.area_points(false)
    }

    fn solve_part2(&self) -> usize {
        self.area_points(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 62);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 952408144115);
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["Z 2 (#015232)", "L2 (#015232)", "L 2 (#01532)", "L"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }

        for input in ["L F (#015232)", "L 2 (#0S5232)"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::ParseInt(_))
            ));
        }
    }
}
