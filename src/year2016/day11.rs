//! # 2016 day 11 - Radioisotope Thermoelectric Generators
//!
//! Basically a BFS solution, where the most important realization to get decent speed is that the isotope names are not
//! important in terms of state space, so instead we throw away the names and store the isotopes as pairs of current
//! floor of its microchip and its generator. (Not doing this abstraction causes the state space to explode with
//! different permutations of isotopes in the same overall state, just different names on the corresponding positions).
//! For part 2, simply add 2 isotopes with both generators and microchips on the first floor and run again.
use crate::aoc_util::error::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

#[derive(PartialEq)]
enum ItemType {
    Microchip,
    Generator,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    elevator_floor: usize,
    isotopes: Vec<Isotope>,
}

impl State {
    /// Returns the possible allowed next states
    fn get_next_states(&self, top_floor: usize) -> Vec<State> {
        let mut current_floor_items = Vec::new();
        let mut new_states = Vec::new();
        for (i, iso) in self.isotopes.iter().enumerate() {
            if iso.microchip_floor == self.elevator_floor {
                current_floor_items.push((i, ItemType::Microchip));
            }
            if iso.generator_floor == self.elevator_floor {
                current_floor_items.push((i, ItemType::Generator));
            }
        }
        // Possible moves: a single item, or a combination of two items
        let mut move_combos: Vec<_> = current_floor_items.iter().map(|item| vec![item]).collect();

        for (i, first) in current_floor_items.iter().enumerate() {
            for second in current_floor_items.iter().skip(i + 1) {
                move_combos.push(vec![first, second]);
            }
        }

        for items in &move_combos {
            if items.len() == 2 && items[0].1 != items[1].1 && items[0].0 != items[1].0 {
                // We can't take a generator and a microchip of different items together on the elevator
                continue;
            }
            for elevator_direction in [-1isize, 1] {
                let newfloor = self.elevator_floor as isize + elevator_direction;
                if newfloor < 0 || newfloor as usize > top_floor {
                    continue;
                }
                let newfloor = newfloor as usize;
                let mut new_isotopes = self.isotopes.clone();
                for (idx, t) in items {
                    match t {
                        ItemType::Generator => {
                            new_isotopes[*idx] = Isotope {
                                microchip_floor: new_isotopes[*idx].microchip_floor,
                                generator_floor: newfloor,
                            };
                        }
                        ItemType::Microchip => {
                            new_isotopes[*idx] = Isotope {
                                microchip_floor: newfloor,
                                generator_floor: new_isotopes[*idx].generator_floor,
                            };
                        }
                    }
                }
                new_isotopes.sort_unstable();
                let new_state = State {
                    elevator_floor: newfloor,
                    isotopes: new_isotopes,
                };
                if new_state.is_valid() {
                    new_states.push(new_state);
                }
            }
        }
        new_states
    }

    /// Checks if current state is valid, i.e. all chips are safe.
    fn is_valid(&self) -> bool {
        let generator_floors: HashSet<_> =
            self.isotopes.iter().map(|i| i.generator_floor).collect();
        self.isotopes
            .iter()
            .all(|iso| iso.is_safe(&generator_floors))
    }

    /// Checks whether current state has reached the end goal, with all objects having reached the top floor
    fn is_complete(&self, top_floor: usize) -> bool {
        self.isotopes
            .iter()
            .all(|iso| iso.microchip_floor == top_floor && iso.generator_floor == top_floor)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Isotope {
    microchip_floor: usize,
    generator_floor: usize,
}

impl Isotope {
    fn new(floors: &[Option<usize>]) -> Result<Self, AocError> {
        Ok(Isotope {
            microchip_floor: floors[1].ctx("microchip floor")?,
            generator_floor: floors[0].ctx("generator floor")?,
        })
    }

    /// A microchip is safe when it shares its floor with its own generator, or when no generator
    /// is present on its floor (an unaccompanied chip among other generators gets fried).
    fn is_safe(&self, generator_floors: &HashSet<usize>) -> bool {
        self.microchip_floor == self.generator_floor
            || !generator_floors.contains(&self.microchip_floor)
    }
}

#[derive(Debug)]
struct InputData {
    isotopes: Vec<Isotope>,
    top_floor: usize,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut top_floor = 0;
        let mut items: HashMap<&str, Vec<Option<usize>>> = HashMap::new();
        for (floor, line) in s.lines().enumerate() {
            top_floor = floor;
            let words: Vec<_> = line
                .split([' ', ',', '.', '-'])
                .skip(4)
                .filter(|w| !w.is_empty() && *w != "compatible")
                .collect();

            for &[w1, w2] in words.array_windows() {
                match w2 {
                    "generator" => {
                        items.entry(w1).or_insert(vec![None, None])[0] = Some(floor);
                    }
                    "microchip" => {
                        items.entry(w1).or_insert(vec![None, None])[1] = Some(floor);
                    }
                    _ => (),
                }
            }
        }
        Ok(Self {
            isotopes: items
                .values()
                .map(|floors| Isotope::new(floors))
                .collect::<Result<Vec<Isotope>, _>>()?,
            top_floor,
        })
    }
}

impl InputData {
    fn get_min_steps(&self, extra_parts: bool) -> Result<usize, AocError> {
        let mut parts = self.isotopes.clone();
        if extra_parts {
            parts.push(Isotope {
                microchip_floor: 0,
                generator_floor: 0,
            });
            parts.push(Isotope {
                microchip_floor: 0,
                generator_floor: 0,
            });
        }
        parts.sort_unstable();
        let mut seen = HashSet::new();
        let state = State {
            elevator_floor: 0,
            isotopes: parts,
        };
        let mut queue: VecDeque<_> = VecDeque::from([(0usize, state)]);
        while let Some((steps, state)) = queue.pop_front() {
            if state.is_complete(self.top_floor) {
                return Ok(steps);
            }
            if seen.contains(&state) {
                continue;
            }
            let next_states = state.get_next_states(self.top_floor);
            seen.insert(state);
            for newstate in next_states {
                if !seen.contains(&newstate) {
                    queue.push_back((steps + 1, newstate));
                }
            }
        }
        Err(AocError::NoSolution)
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        self.get_min_steps(false)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        self.get_min_steps(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    const TEST_OFFICIAL_2: &str = "The first floor contains a lithium generator and a lithium-compatible microchip.
The second floor contains a hydrogen generator and an elerium generator and a dilithium generator.
The third floor contains a hydrogen-compatible microchip and an elerium-compatible microchip and a dilithium-compatible microchip.
The fourth floor contains nothing relevant.";

    // Unsolvable for part 1
    const TEST_CUSTOM_1: &str = "The first floor contains nothing relevant.
The second floor contains a hydrogen generator, and a lithium generator.
The third floor contains a hydrogen-compatible microchip, and a lithium-compatible microchip.
The fourth floor contains nothing relevant.";

    const TEST_CUSTOM_2: &str =
        "The first floor contains a hydrogen generator and a lithium-compatible microchip.
The second floor contains a hydrogen-compatible microchip and a lithium generator.
The third floor contains nothing relevant.";

    const TEST_PARSER_1: &str = "The first floor contains a hydrogen-compatible microchip.
The second floor contains a hydrogen generator, and a lithium generator.
The third floor contains nothing relevant.";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 11);
    }

    #[test]
    fn part1_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 25);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(
            solution_data.solve_part1().unwrap_err(),
            AocError::NoSolution
        );
    }

    #[test]
    fn part1_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 8);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 49);
    }

    #[test]
    fn parser_missing_microchip() {
        let err = InputData::from_str(TEST_PARSER_1).unwrap_err();
        assert_eq!(err, AocError::Missing("microchip floor"));
    }
}
