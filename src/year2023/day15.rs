//! # 2023 day 15 - Lens Library
//!
//! Each comma-separated step is hashed for part 1 while being parsed into a borrowed `Operation`.
//! The label's own hash is stored alongside it so part 2 can index the 256 boxes without re-hashing.
//! Labels borrow from the input string, avoiding a heap allocation per step.
use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const HASH_WIDTH: u32 = 256;

fn hash_algorithm(input: &str) -> usize {
    input
        .bytes()
        .fold(0u32, |h, c| ((h + u32::from(c)) * 17) % HASH_WIDTH) as usize
}

enum Operation<'a> {
    Remove {
        label: &'a str,
        hashed: usize,
    },
    Replace {
        label: &'a str,
        strength: u32,
        hashed: usize,
    },
}

impl<'a> Operation<'a> {
    fn parse(s: &'a str) -> Result<Self, AocError> {
        if let Some(label) = s.strip_suffix('-') {
            Ok(Self::Remove {
                label,
                hashed: hash_algorithm(label),
            })
        } else if let Some((label, strength)) = s.split_once('=') {
            Ok(Self::Replace {
                label,
                strength: strength.parse()?,
                hashed: hash_algorithm(label),
            })
        } else {
            Err(AocError::Invalid(format!("invalid operation: {s}")))
        }
    }
}

struct Lens<'b> {
    label: &'b str,
    strength: u32,
}

struct InputData<'a> {
    hash_sum: usize,
    steps: Vec<Operation<'a>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut hash_sum = 0;
        let mut steps = Vec::new();
        for step in s.split(',') {
            hash_sum += hash_algorithm(step);
            steps.push(Operation::parse(step)?);
        }
        Ok(Self { hash_sum, steps })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        self.hash_sum
    }

    fn solve_part2(&self) -> usize {
        let mut boxes: Vec<Vec<Lens<'_>>> = Vec::new();
        boxes.resize_with(HASH_WIDTH as usize, Vec::new);

        for step in &self.steps {
            match step {
                Operation::Replace {
                    label,
                    strength,
                    hashed,
                } => {
                    if let Some(idx) = boxes[*hashed].iter().position(|lens| lens.label == *label) {
                        boxes[*hashed][idx].strength = *strength;
                    } else {
                        boxes[*hashed].push(Lens {
                            label,
                            strength: *strength,
                        });
                    }
                }
                Operation::Remove { label, hashed } => {
                    if let Some(idx) = boxes[*hashed].iter().position(|lens| lens.label == *label) {
                        boxes[*hashed].remove(idx);
                    }
                }
            }
        }

        boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, lens)| (i + 1) * (j + 1) * lens.strength as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_hash() {
        assert_eq!(hash_algorithm("HASH"), 52);
    }

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 1320);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 145);
    }

    #[test]
    fn parser_invalid_operation() {
        assert!(matches!(
            InputData::try_from("rn?"),
            Err(AocError::Invalid(_))
        ));
    }

    #[test]
    fn parser_invalid_strength() {
        assert!(matches!(
            InputData::try_from("rn=x"),
            Err(AocError::ParseInt(_))
        ));
    }
}
