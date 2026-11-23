//! # 2025 day 10 - Factory
//!
//! Stores the input in bit format as integers and uses XOR operations to perform the button
//! presses. For every machine (row) in the input, a solution map is generated, where every
//! possible combination of button presses are mapped to the resulting target mask.
//!
//! ## Part 1
//!
//! Simply get the possible solutions for the target mask matching the light diagram, and then
//! choose the one with the fewest button presses, i.e. the binary value with the least number of
//! ones.
//!
//! ## Part 2
//!
//! Uses a recursive approach, inspired by
//! [this idea](https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/).
//! The solution map is used again here to avoid recalculating the same solutions over and over.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct Machine {
    light_diagram: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
    solution_map: HashMap<u32, Vec<u32>>,
}

impl FromStr for Machine {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let ld = parts.next().ctx("missing light diagram")?;
        let j = parts.next_back().ctx("missing joltage")?;

        let lights = ld
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ctx("invalid light diagram")?;
        if lights.len() > 32 {
            return Err(AocError::Invalid("too many indicator lights".into()));
        }
        let mut light_diagram = 0u32;
        for (i, c) in lights.bytes().enumerate() {
            match c {
                b'#' => light_diagram |= 1 << i,
                b'.' => {}
                _ => {
                    return Err(AocError::Invalid(format!(
                        "invalid light character: {}",
                        c as char
                    )));
                }
            }
        }

        let mut buttons = Vec::new();
        for part in parts {
            let indices = part
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .ctx("invalid button specification")?;
            let mut mask = 0u32;
            for n in indices.split(',') {
                let idx: u32 = n.parse()?;
                if idx >= 32 {
                    return Err(AocError::Invalid(format!(
                        "button index out of range: {idx}"
                    )));
                }
                mask |= 1 << idx;
            }
            buttons.push(mask);
        }
        if buttons.len() > 32 {
            return Err(AocError::Invalid("too many buttons".into()));
        }

        let values = j
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ctx("invalid joltage specification")?;
        let joltage: Vec<u32> = values.split(',').map(|n| n.parse()).collect::<Result<_, _>>()?;
        if joltage.len() != lights.len() {
            return Err(AocError::Invalid(format!(
                "expected {} joltage values, got {}",
                lights.len(),
                joltage.len()
            )));
        }

        // Build a map of button press solutions for every possible target mask
        let mut solution_map: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut queue = VecDeque::from([(0, 0)]);
        while let Some((target_mask, buttons_pressed)) = queue.pop_front() {
            solution_map
                .entry(target_mask)
                .and_modify(|e| e.push(buttons_pressed))
                .or_insert(vec![buttons_pressed]);
            // Generate new combinations by pushing one more button beyond the highest button index pressed so far
            // (We're evaluating by adding from low to high, so any combination with a lower index button will
            // already have been explored.)
            let idx_highest_button_pressed = (32 - buttons_pressed.leading_zeros()) as usize;
            for (i, button) in buttons.iter().enumerate().skip(idx_highest_button_pressed) {
                queue.push_back((target_mask ^ button, (1 << i) | buttons_pressed));
            }
        }

        Ok(Self {
            light_diagram,
            buttons,
            joltage,
            solution_map,
        })
    }
}

impl Machine {
    fn get_indicator_light_presses_least_cost(&self, target: u32) -> Option<u32> {
        if let Some(solutions) = self.solution_map.get(&target) {
            solutions.iter().map(|s| s.count_ones()).min()
        } else {
            None
        }
    }

    fn get_joltage_presses_total(&self) -> Option<u32> {
        self.get_joltage_presses(&self.joltage)
    }

    fn get_joltage_presses(&self, joltage: &[u32]) -> Option<u32> {
        if joltage.iter().all(|v| *v == 0) {
            return Some(0);
        }
        let odd_mask: u32 = joltage.iter().enumerate().map(|(i, n)| (*n % 2) << i).sum();
        let mut best: Option<u32> = None;
        if let Some(solutions) = self.solution_map.get(&odd_mask) {
            for buttons_pressed in solutions {
                if let Some(remainder) = self.get_joltage_remainder(*buttons_pressed, joltage)
                    && let Some(rem_cost) = self.get_joltage_presses(&remainder)
                {
                    let cost = buttons_pressed.count_ones() + (2 * rem_cost);
                    if let Some(b) = best {
                        best = Some(b.min(cost))
                    } else {
                        best = Some(cost);
                    }
                }
            }
        }
        best
    }

    fn get_joltage_remainder(&self, presses: u32, joltage: &[u32]) -> Option<Vec<u32>> {
        let mut new_joltages = joltage.to_vec();
        for (idx, button) in self.buttons.iter().enumerate() {
            if (presses >> idx) & 1 > 0 {
                for (i, jolt) in new_joltages.iter_mut().enumerate() {
                    if (*button >> i) & 1 > 0 {
                        if *jolt == 0 {
                            return None;
                        }
                        *jolt -= 1;
                    }
                }
            }
        }
        for jolt in new_joltages.iter_mut() {
            *jolt /= 2;
        }
        Some(new_joltages)
    }
}

struct InputData {
    machines: Vec<Machine>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            machines: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<u32, AocError> {
        self.machines.iter().try_fold(0u32, |total, machine| {
            let presses = machine
                .get_indicator_light_presses_least_cost(machine.light_diagram)
                .ok_or(AocError::NoSolution)?;
            Ok(total + presses)
        })
    }

    fn solve_part2(&self) -> Result<u32, AocError> {
        self.machines.iter().try_fold(0u32, |total, machine| {
            let presses = machine
                .get_joltage_presses_total()
                .ok_or(AocError::NoSolution)?;
            Ok(total + presses)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    const TEST_CUSTOM_1: &str = "[##..##..] (0,1,3,4,5,6) (2,4,6) (1,5,6,7) (0,4,5) (3,6) (1,2,3,5,6) (0,2,4,5,6,7) {31,34,131,26,148,48,168,25}";
    const TEST_CUSTOM_2: &str = "[.##...#.] (4,5) (0,1,2,3,7) (0,1,2,3,6) (0,1,2,4,6,7) (3,4,7) (0,3,6,7) {64,50,50,174,163,15,50,176}";
    const TEST_CUSTOM_3: &str = "[.##..] (1,2) (0,2,3,4) (0,1,3,4) {17,20,19,17,17}";
    const TEST_CUSTOM_4: &str = "[###] (0,1) (0,2) (1,2) {2,2,2}";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), Ok(7));
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(33));
    }

    // Some extra test of problematic inputs
    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(171));
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(208));
    }

    #[test]
    fn part2_custom_3() {
        let solution_data: InputData = TEST_CUSTOM_3.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(28));
    }

    #[test]
    fn part2_custom_4() {
        let solution_data: InputData = TEST_CUSTOM_4.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Ok(3));
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "[.##.] (1)",             // missing joltage
            "(.##.) (1) {1,2,3,4,5}", // wrong light brackets
            "[.?.?] (1) {1,2,3,4,5}", // invalid light character
            "[.##.] (99) {1,2,3,4,5}", // button index out of range
            "[.##.] (1) {1,2,x,4,5}", // bad joltage value
            "[.##.] (1) {1,2,3}",     // joltage width mismatch
        ] {
            assert!(
                input.parse::<Machine>().is_err(),
                "expected error for {input:?}"
            );
        }
    }

    #[test]
    fn solvers_no_solution() {
        let data = InputData {
            machines: vec![Machine {
                light_diagram: 1,
                buttons: vec![],
                joltage: vec![1],
                solution_map: HashMap::new(),
            }],
        };
        assert_eq!(data.solve_part1(), Err(AocError::NoSolution));
        assert_eq!(data.solve_part2(), Err(AocError::NoSolution));
    }
}
