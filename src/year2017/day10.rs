//! # 2017 day 10 - Knot Hash
use std::error::Error;

use super::knothash::*;
use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1(STANDARD_BUFFER_SIZE)?.to_string(),
        solution_data.solve_part2(STANDARD_BUFFER_SIZE),
    ))
}

const STANDARD_BUFFER_SIZE: usize = 256;

/// Stores the raw input, since the full string is needed for part 2. And since some test vectors
/// for part 2 don't follow the "1,2,3" pattern, the parsing for part 1 needs to be done in the
/// solver function for that input to work.
struct InputData<'a> {
    raw_input: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { raw_input: s })
    }
}

impl InputData<'_> {
    fn solve_part1(&self, buffer_len: usize) -> Result<usize, AocError> {
        let lengths: Vec<_> = self
            .raw_input
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let nbrs = generate_hash(&lengths, buffer_len, 1);
        Ok(nbrs.iter().take(2).product())
    }

    fn solve_part2(&self, buffer_len: usize) -> String {
        get_knot_hash(self.raw_input, buffer_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from("3,4,1,5").unwrap();
        assert_eq!(solution_data.solve_part1(5).unwrap(), 12);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from("").unwrap();
        assert_eq!(
            solution_data.solve_part2(STANDARD_BUFFER_SIZE),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from("AoC 2017").unwrap();
        assert_eq!(
            solution_data.solve_part2(STANDARD_BUFFER_SIZE),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
    }

    #[test]
    fn part2_official_3() {
        let solution_data = InputData::try_from("1,2,3").unwrap();
        assert_eq!(
            solution_data.solve_part2(STANDARD_BUFFER_SIZE),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
    }

    #[test]
    fn part2_official_4() {
        let solution_data = InputData::try_from("1,2,4").unwrap();
        assert_eq!(
            solution_data.solve_part2(STANDARD_BUFFER_SIZE),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
