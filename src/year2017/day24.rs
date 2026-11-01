//! # 2017 day 24 - Electromagnetic Moat
//!
//! Uses DFS to try to assemble all possible combinations of components. The main optimization is to
//! always choose a component with two identical ports when possible and skip exploring other options
//! at that point in the branch; it's free strength and length without changing the open port, so
//! there is never a reason not to use it, and there's no benefit in saving it for later. This is done
//! by first sorting the input such that those components are first in the vector, and then the search
//! can break early when evaluating the list during the search.
//!
//! Since the DFS explores all possibilities, we can get the answer for both parts in one pass, just
//! by tracking the necessary parameters for both parts.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug)]
struct Component {
    ports: (u32, u32),
}

impl FromStr for Component {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('/').ctx("separator")?;
        Ok(Self {
            ports: (left.parse()?, right.parse()?),
        })
    }
}

impl Component {
    /// Checks if the input port matches any of its ports. If it does, it returns its other
    /// remaining port, otherwise None.
    #[inline]
    fn try_connect(&self, port: u32) -> Option<u32> {
        if self.ports.0 == port {
            Some(self.ports.1)
        } else if self.ports.1 == port {
            Some(self.ports.0)
        } else {
            None
        }
    }
}

struct State {
    open_port: u32,
    strength: u32,
    length: u32,
}

impl State {
    fn add_component(&self, new_port: u32) -> Self {
        Self {
            open_port: new_port,
            strength: self.strength + self.open_port + new_port,
            length: self.length + 1,
        }
    }
}

#[derive(Debug)]
struct InputData {
    components: Vec<Component>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components: Vec<Component> = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        // Sort the input so the components with two identical ports are first in the list
        components.sort_unstable_by_key(|c| if c.ports.0 == c.ports.1 { 0 } else { 1 });
        Ok(Self { components })
    }
}

impl InputData {
    fn dfs(
        &self,
        state: &State,
        used: &mut [bool],
        best_strength_p1: &mut u32,
        best_strength_p2: &mut u32,
        best_length: &mut u32,
    ) {
        // 1. Update your global/accumulated best score(s) at each step
        *best_strength_p1 = (*best_strength_p1).max(state.strength);

        // 2. Update Part 2 (Longest first, then strongest)
        if state.length > *best_length {
            *best_length = state.length;
            *best_strength_p2 = state.strength;
        } else if state.length == *best_length {
            *best_strength_p2 = (*best_strength_p2).max(state.strength);
        }

        // 2. Iterate through all available components
        for (i, c) in self.components.iter().enumerate() {
            if used[i] {
                continue; // Skip already used components
            }

            if let Some(next_port) = c.try_connect(state.open_port) {
                used[i] = true; // Mark as used
                let next_state = state.add_component(next_port);

                self.dfs(
                    &next_state,
                    used,
                    best_strength_p1,
                    best_strength_p2,
                    best_length,
                );

                used[i] = false; // Un-mark (Backtrack)

                if c.ports.0 == c.ports.1 {
                    // If we used a component with two identical ports, we already know we made the best choice for this branch
                    break;
                }
            }
        }
    }

    fn solve(&self) -> (u32, u32) {
        let start = State {
            open_port: 0,
            strength: 0,
            length: 0,
        };
        let mut strength_p1 = 0;
        let mut strength_p2 = 0;
        let mut length = 0;
        let mut used = vec![false; self.components.len()];
        self.dfs(
            &start,
            &mut used,
            &mut strength_p1,
            &mut strength_p2,
            &mut length,
        );
        (strength_p1, strength_p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    // Two disjoint chains starting at 0. The stronger one is found after the weaker one of the
    // same (maximal) length, forcing the strength tie-break when tracking the longest bridge.
    const TEST_CUSTOM_1: &str = "0/5
5/6
6/7
0/1
1/2
2/3";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 31);
        assert_eq!(p2, 19);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 29);
        assert_eq!(p2, 29);
    }

    #[test]
    fn parser_invalid_input() {
        let err = InputData::from_str("56").unwrap_err();
        assert_eq!(err, AocError::Missing("separator"));
        let err = InputData::from_str("5/6f").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
