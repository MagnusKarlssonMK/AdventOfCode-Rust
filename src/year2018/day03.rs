//! # 2018 day 3 - No Matter How You Slice It
use crate::aoc_util::point::Point;
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct Claim {
    id: usize,
    corner: Point,
    x_len: usize,
    y_len: usize,
}

impl FromStr for Claim {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .strip_prefix('#')
            .unwrap()
            .split(&['@', ',', ':', 'x'])
            .map(|n| n.trim().parse::<usize>().unwrap());
        Ok(Self {
            id: parts.next().unwrap(),
            corner: Point::new(parts.next().unwrap() as i32, parts.next().unwrap() as i32),
            x_len: parts.next().unwrap(),
            y_len: parts.next().unwrap(),
        })
    }
}

struct InputData {
    claims: Vec<Claim>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            claims: s
                .lines()
                .map(|line| Claim::from_str(line).unwrap())
                .collect(),
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
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
        let mut p2 = 0;

        for c in possible.iter().map(|i| self.claims.get(*i).unwrap()) {
            let mut overlap = false;
            for p in (0..c.x_len).flat_map(|dx| {
                (0..c.y_len).map(move |dy| Point::new(dx as i32, dy as i32) + c.corner)
            }) {
                if *seen.get(&p).unwrap() > 1 {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                p2 = c.id;
                break;
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn parts1_2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 4);
        assert_eq!(p2, 3);
    }
}
