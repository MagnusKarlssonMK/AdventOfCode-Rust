//! # 2018 day 2 - Inventory Management System
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    ids: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            ids: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        let mut twos: usize = 0;
        let mut threes: usize = 0;
        for id in &self.ids {
            let mut counts: HashMap<char, usize> = HashMap::new();
            let mut two: isize = 0;
            let mut three: isize = 0;
            for c in id.chars() {
                if let Some(x) = counts.get_mut(&c) {
                    match *x {
                        1 => two += 1,
                        2 => {
                            two -= 1;
                            three += 1
                        }
                        3 => three -= 1,
                        _ => (),
                    }
                    *x += 1;
                } else {
                    counts.insert(c, 1);
                }
            }
            if two > 0 {
                twos += 1;
            }
            if three > 0 {
                threes += 1;
            }
        }
        twos * threes
    }

    fn solve_part2(&self) -> String {
        for left in 0..self.ids.len() - 1 {
            for right in 1..self.ids.len() {
                let common: String = self.ids[left]
                    .chars()
                    .zip(self.ids[right].chars())
                    .filter_map(|(lc, rc)| if lc == rc { Some(lc) } else { None })
                    .collect();
                if common.len() == self.ids[left].len() - 1 {
                    return common;
                }
            }
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), "fgij");
    }
}
