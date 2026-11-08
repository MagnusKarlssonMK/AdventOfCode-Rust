//! # 2020 day 13 - Shuttle Search
//!
//! Simply finds the first bus departing at or after the estimate for part 1.
//! Chinese remainder theorem solution for part 2.
use crate::aoc_util::{
    error::{AocError, OptionExt},
    math::chinese_remainder,
};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct InputData {
    estimate: u128,
    buslist: Vec<(u128, u128)>, // busid, offset-idx
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_bus(s: &str, i: usize) -> Result<(u128, u128), AocError> {
            let v = s.parse()?;
            if v != 0 {
                Ok((v, i as u128))
            } else {
                Err(AocError::Invalid(s.to_string()))
            }
        }
        let mut lines = s.lines();
        let estimate = lines.next().ctx("estimate")?.parse()?;
        let buslist = lines
            .next()
            .ctx("buslist")?
            .split(',')
            .enumerate()
            .filter(|(_, v)| *v != "x")
            .map(|(i, v)| parse_bus(v, i))
            .collect::<Result<_, _>>()?;
        Ok(Self { estimate, buslist })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<u128, AocError> {
        let (wait, id) = self
            .buslist
            .iter()
            .map(|(id, _)| ((id - self.estimate % id) % id, *id))
            .min()
            .ok_or(AocError::NoSolution)?;
        Ok(wait * id)
    }

    fn solve_part2(&self) -> Result<u128, AocError> {
        let mut moduli = Vec::with_capacity(self.buslist.len());
        let mut remainders = Vec::with_capacity(self.buslist.len());

        for &(busid, idx) in &self.buslist {
            moduli.push(busid);
            remainders.push((busid - idx % busid) % busid);
        }

        let time = chinese_remainder(&moduli, &remainders).map_err(|_| AocError::NoSolution)?;
        u128::try_from(time).map_err(|_| AocError::NoSolution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "939
7,13,x,x,59,x,31,19";

    /// For part 1: bus departs at exactly the estimate
    const TEST_CUSTOM_1: &str = "100\n50";
    /// Input with no active buses
    const TEST_CUSTOM_2: &str = "10\nx,x,x";
    /// Input with inactive bus in first slot
    const TEST_CUSTOM_3: &str = "0\nx,5";
    /// Input with inactive bus in first slot
    const TEST_CUSTOM_4: &str = "0\n5,x,x,x,x,x,3";
    /// Pairwise coprime not fulfilled, 1st variant
    const TEST_CUSTOM_5: &str = "0\n6,3";
    /// Pairwise coprime not fulfilled, 2nd variant
    const TEST_CUSTOM_6: &str = "0\n6,10";
    /// Consistent non-coprime, idx 3
    const TEST_CUSTOM_7: &str = "0\n6,x,x,3";
    /// Consistent non-coprime, idx 8
    const TEST_CUSTOM_8: &str = "0\n6,x,x,x,x,x,x,x,10";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 295);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 1068781);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 0);
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), Err(AocError::NoSolution));
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn part1_2_custom_3() {
        let solution_data: InputData = TEST_CUSTOM_3.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 0);
        assert_eq!(solution_data.solve_part2().unwrap(), 4);
    }

    #[test]
    fn part1_2_custom_4() {
        let solution_data: InputData = TEST_CUSTOM_4.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 0);
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }

    #[test]
    fn part2_custom_5() {
        let solution_data: InputData = TEST_CUSTOM_5.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn part2_custom_6() {
        let solution_data: InputData = TEST_CUSTOM_6.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), Err(AocError::NoSolution));
    }

    #[test]
    fn part2_custom_7() {
        let solution_data: InputData = TEST_CUSTOM_7.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 0);
    }

    #[test]
    fn part2_custom_8() {
        let solution_data: InputData = TEST_CUSTOM_8.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 12);
    }
}
