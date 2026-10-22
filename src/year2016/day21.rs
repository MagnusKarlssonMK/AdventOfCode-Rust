//! # 2016 day 21 - Scrambled Letters and Hash
//!
//! Part 1 applies the scrambling operations in order. Part 2 walks the operations
//! in reverse order and applies the inverse of each one. All operations are
//! trivially invertible except "rotate based on position of letter", whose rotation
//! amount depends on the letter's index *before* the rotation, so its inverse has to
//! be searched for.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1(PART_1_PASSWORD)?,
        solution_data.solve_part2(PART_2_PASSWORD)?,
    ))
}

const PART_1_PASSWORD: &str = "abcdefgh";
const PART_2_PASSWORD: &str = "fbgdceah";

#[derive(Clone, Copy, Debug)]
enum Operation {
    /// SwapPosition(X, Y): Letters at indexes X and Y (counted from 0) should be swapped.
    SwapPosition(usize, usize),
    /// SwapLetter(X, Y): Letters X and Y should be swapped, regardless of where they appear in the string.
    SwapLetter(char, char),
    /// RotateLeft(X): The whole string should be rotated X steps left
    RotateLeft(usize),
    /// RotateRight(Y): The whole string should be rotated X steps right
    RotateRight(usize),
    /// RotatePositionLeft(X): The whole string should be rotated to the left based on the index of letter X
    RotatePositionLeft(char),
    /// RotatePositionRight(X): The whole string should be rotated to the right based on the index of letter X
    RotatePositionRight(char),
    /// ReversePosition(X, Y): The span of letters at indexes X through Y (including the letters at X and Y) should be reversed in order.
    ReversePosition(usize, usize),
    /// Move(X, Y): The letter which is at index X should be removed from the string, then inserted such that it ends up at index Y.
    Move(usize, usize),
}

impl FromStr for Operation {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = |word: &str| {
            word.parse::<char>()
                .map_err(|_| AocError::Invalid(s.to_string()))
        };
        let words: Vec<_> = s.split_ascii_whitespace().collect();
        match words.as_slice() {
            ["swap", "position", x, "with", "position", y] => Ok(Operation::SwapPosition(
                x.parse::<usize>()?,
                y.parse::<usize>()?,
            )),
            ["swap", "letter", x, "with", "letter", y] => {
                Ok(Operation::SwapLetter(letter(x)?, letter(y)?))
            }
            ["rotate", "left", x, _] => Ok(Operation::RotateLeft(x.parse::<usize>()?)),
            ["rotate", "right", x, _] => Ok(Operation::RotateRight(x.parse::<usize>()?)),
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                Ok(Operation::RotatePositionRight(letter(x)?))
            }
            ["reverse", "positions", x, "through", y] => Ok(Operation::ReversePosition(
                x.parse::<usize>()?,
                y.parse::<usize>()?,
            )),
            ["move", "position", x, "to", "position", y] => {
                Ok(Operation::Move(x.parse::<usize>()?, y.parse::<usize>()?))
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Operation {
    /// Rotation amount for "rotate based on position of letter": one step, plus
    /// one more when the letter is at index 4 or later.
    fn position_rotation(index: usize) -> usize {
        index + 1 + usize::from(index >= 4)
    }

    fn find_letter(
        password: &[char],
        letter: char,
        operation: &Operation,
    ) -> Result<usize, AocError> {
        password
            .iter()
            .position(|c| *c == letter)
            .ok_or_else(|| AocError::Invalid(format!("rotate letter not found: {:?}", operation)))
    }

    fn process(&self, password: &mut [char]) -> Result<(), AocError> {
        let size = password.len();
        match self {
            Operation::SwapPosition(x, y) => {
                if *x >= size || *y >= size {
                    return Err(AocError::Invalid(format!("swap out of range: {:?}", self)));
                }
                password.swap(*x, *y);
            }
            Operation::SwapLetter(x, y) => password.iter_mut().for_each(|c| {
                if *c == *x {
                    *c = *y;
                } else if *c == *y {
                    *c = *x;
                }
            }),
            Operation::RotateLeft(x) => password.rotate_left(*x % size),
            Operation::RotateRight(x) => password.rotate_right(*x % size),
            Operation::RotatePositionRight(x) => {
                let index = Self::find_letter(password, *x, self)?;
                password.rotate_right(Self::position_rotation(index) % size);
            }
            Operation::RotatePositionLeft(x) => {
                // The rotation amount depends on the index *before* rotating, so the
                // inverse can't be computed in a single step. Try every rotation and
                // keep the one that the forward rule would have produced from.
                let mut candidate = password.to_vec();
                for steps in 0..size {
                    candidate.copy_from_slice(password);
                    candidate.rotate_left(steps);
                    let index = Self::find_letter(&candidate, *x, self)?;
                    if Self::position_rotation(index) % size == steps {
                        password.copy_from_slice(&candidate);
                        return Ok(());
                    }
                }
                return Err(AocError::NoSolution);
            }
            Operation::ReversePosition(x, y) => password
                .get_mut(*x..=*y)
                .ok_or(AocError::Invalid(format!(
                    "reverse out of range: {:?}",
                    self
                )))?
                .reverse(),
            Operation::Move(x, y) => {
                if x < y {
                    password
                        .get_mut(*x..=*y)
                        .ok_or(AocError::Invalid(format!("move out of range: {:?}", self)))?
                        .rotate_left(1);
                } else {
                    password
                        .get_mut(*y..=*x)
                        .ok_or(AocError::Invalid(format!("move out of range: {:?}", self)))?
                        .rotate_right(1);
                }
            }
        }
        Ok(())
    }

    fn get_reverse(&self) -> Operation {
        match self {
            Self::RotateLeft(x) => Self::RotateRight(*x),
            Self::RotateRight(x) => Self::RotateLeft(*x),
            Self::RotatePositionLeft(x) => Self::RotatePositionRight(*x),
            Self::RotatePositionRight(x) => Self::RotatePositionLeft(*x),
            Self::Move(x, y) => Self::Move(*y, *x),
            _ => *self,
        }
    }
}

#[derive(Debug)]
struct InputData {
    operations: Vec<Operation>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            operations: s
                .lines()
                .map(Operation::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self, password: &str) -> Result<String, AocError> {
        let mut password: Vec<_> = password.chars().collect();
        for operation in &self.operations {
            operation.process(&mut password)?;
        }
        Ok(password.iter().collect())
    }

    fn solve_part2(&self, password: &str) -> Result<String, AocError> {
        let mut password: Vec<_> = password.chars().collect();
        for operation in self.operations.iter().rev() {
            operation.get_reverse().process(&mut password)?;
        }
        Ok(password.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    /// The step-by-step example from the puzzle description, one case per
    /// operation, in the order they are applied.
    const OPERATION_STEPS: &[(&str, &str, &str)] = &[
        ("swap position 4 with position 0", "abcde", "ebcda"),
        ("swap letter d with letter b", "ebcda", "edcba"),
        ("reverse positions 0 through 4", "edcba", "abcde"),
        ("rotate left 1 step", "abcde", "bcdea"),
        ("move position 1 to position 4", "bcdea", "bdeac"),
        ("move position 3 to position 0", "bdeac", "abdec"),
        ("rotate based on position of letter b", "abdec", "ecabd"),
        ("rotate based on position of letter d", "ecabd", "decab"),
    ];

    /// Apply one operation, either forwards or through its inverse.
    fn apply(operation: &str, password: &str, inverse: bool) -> String {
        let mut password: Vec<_> = password.chars().collect();
        let operation = Operation::from_str(operation).unwrap();
        if inverse {
            operation.get_reverse().process(&mut password).unwrap();
        } else {
            operation.process(&mut password).unwrap();
        }
        password.iter().collect()
    }

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1("abcde").unwrap(), "decab");
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2("decab").unwrap(), "abcde");
    }

    #[test]
    fn operations_forward() {
        for &(operation, before, after) in OPERATION_STEPS {
            assert_eq!(apply(operation, before, false), after, "{operation}");
        }
    }

    #[test]
    fn operation_inverses() {
        // The eight letter password makes every inverse unique, including the
        // rotate-based-on-position one which is ambiguous for the five letter
        // sample above.
        for &(operation, _, _) in OPERATION_STEPS {
            let scrambled = apply(operation, PART_1_PASSWORD, false);
            assert_eq!(
                apply(operation, &scrambled, true),
                PART_1_PASSWORD,
                "{operation}"
            );
        }
    }

    #[test]
    fn rotate_position_round_trip() {
        for letter in PART_1_PASSWORD.chars() {
            let operation = format!("rotate based on position of letter {letter}");
            let scrambled = apply(&operation, PART_1_PASSWORD, false);
            assert_eq!(
                apply(&operation, &scrambled, true),
                PART_1_PASSWORD,
                "{operation}"
            );
        }
    }

    #[test]
    fn rotate_wraps() {
        assert_eq!(
            apply("rotate left 9 steps", PART_1_PASSWORD, false),
            "bcdefgha"
        );
        assert_eq!(
            apply("rotate right 9 steps", PART_1_PASSWORD, false),
            "habcdefg"
        );
    }

    #[test]
    fn swap_position_out_of_range() {
        let mut password: Vec<_> = PART_1_PASSWORD.chars().collect();
        let err = Operation::from_str("swap position 0 with position 8")
            .unwrap()
            .process(&mut password)
            .unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)), "{err:?}");
    }

    #[test]
    fn rotate_missing_letter() {
        let mut password: Vec<_> = PART_1_PASSWORD.chars().collect();
        let err = Operation::from_str("rotate based on position of letter z")
            .unwrap()
            .process(&mut password)
            .unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)), "{err:?}");
    }

    #[test]
    fn parser_all_operations() {
        assert!(matches!(
            Operation::from_str("swap position 4 with position 0").unwrap(),
            Operation::SwapPosition(4, 0)
        ));
        assert!(matches!(
            Operation::from_str("swap letter d with letter b").unwrap(),
            Operation::SwapLetter('d', 'b')
        ));
        assert!(matches!(
            Operation::from_str("rotate left 1 step").unwrap(),
            Operation::RotateLeft(1)
        ));
        assert!(matches!(
            Operation::from_str("rotate right 4 steps").unwrap(),
            Operation::RotateRight(4)
        ));
        assert!(matches!(
            Operation::from_str("rotate based on position of letter a").unwrap(),
            Operation::RotatePositionRight('a')
        ));
        assert!(matches!(
            Operation::from_str("reverse positions 0 through 4").unwrap(),
            Operation::ReversePosition(0, 4)
        ));
        assert!(matches!(
            Operation::from_str("move position 1 to position 4").unwrap(),
            Operation::Move(1, 4)
        ));
    }

    #[test]
    fn parser_unknown_operation() {
        let err = Operation::from_str("reverse positions 0 to 4").unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)), "{err:?}");
        let err =
            InputData::from_str("swap position 4 with position 0\ntwiddle thumbs").unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)), "{err:?}");
    }

    #[test]
    fn parser_noninteger() {
        let err = Operation::from_str("rotate left x steps").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)), "{err:?}");
    }

    #[test]
    fn parser_nonletter() {
        let err = Operation::from_str("swap letter dd with letter b").unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)), "{err:?}");
    }
}
