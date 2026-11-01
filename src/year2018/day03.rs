//! # 2018 day 3 - No Matter How You Slice It
//!
//! Simply stores the claimed squares as packed coordinates in a counter dict, and gets the answer
//! to Part 1 by counting how many squares have been counted more than once. While doing that, mark
//! up the claims that don't have overlap just to prune what we need to check for Part 2. Then go
//! over those once more and find which one still doesn't have overlap after the entire dict has
//! been built.
//!
//! Not the most efficient solution since it stores and iterates over every single coordinate. It could
//! be possible to create a solution just based on corners, but it would be MUCH more complicated.
use crate::aoc_util::{error::*, point::Point};
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

struct Claim {
    id: usize,
    corner: Point,
    x_len: usize,
    y_len: usize,
}

impl FromStr for Claim {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .strip_prefix('#')
            .ctx("claim '#' prefix")?
            .split(&['@', ',', ':', 'x'])
            .map(|n| n.trim().parse::<usize>().map_err(AocError::from));
        Ok(Self {
            id: parts.next().ctx("claim id")??,
            corner: Point::new(
                parts.next().ctx("corner x")?? as i32,
                parts.next().ctx("corner y")?? as i32,
            ),
            x_len: parts.next().ctx("x length")??,
            y_len: parts.next().ctx("y length")??,
        })
    }
}

struct InputData {
    claims: Vec<Claim>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            claims: s.lines().map(Claim::from_str).collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut seen = HashMap::new();
        let mut possible = Vec::new();
        for (i, c) in self.claims.iter().enumerate() {
            let mut overlap = false;
            for p in (0..c.x_len).flat_map(|dx| {
                (0..c.y_len).map(move |dy| Point::new(dx as i32, dy as i32) + c.corner)
            }) {
                seen.entry(p)
                    .and_modify(|v| {
                        overlap = true;
                        *v += 1
                    })
                    .or_insert(1);
            }
            if !overlap {
                possible.push(i);
            }
        }
        let p1 = seen.values().filter(|v| **v > 1).count();
        let mut p2 = None;

        for c in possible.iter().map(|i| &self.claims[*i]) {
            let mut overlap = false;
            for p in (0..c.x_len).flat_map(|dx| {
                (0..c.y_len).map(move |dy| Point::new(dx as i32, dy as i32) + c.corner)
            }) {
                if let Some(v) = seen.get(&p)
                    && *v > 1
                {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                p2 = Some(c.id);
                break;
            }
        }
        Ok((p1, p2.ok_or(AocError::NoSolution)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    /// All claims overlap, i.e. no solution for part 2
    const TEST_CUSTOM_1: &str = "#1 @ 0,0: 2x2
#2 @ 0,0: 1x1
#3 @ 1,1: 1x1";

    #[test]
    fn parts1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 4);
        assert_eq!(p2, 3);
    }

    #[test]
    fn parts1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        assert_eq!(solution_data.solve().unwrap_err(), AocError::NoSolution);
    }
}
