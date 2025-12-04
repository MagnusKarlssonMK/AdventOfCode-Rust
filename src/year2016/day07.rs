//! # 2016 day 7 -Internet Protocol Version 7
use std::{collections::HashSet, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

enum Ipv7seq<'a> {
    Hypernet(&'a str),
    Supernet(&'a str),
}

struct Ipv7addr<'a> {
    seqs: Vec<Ipv7seq<'a>>,
}

impl<'a> TryFrom<&'a str> for Ipv7addr<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            seqs: s
                .split(&['[', ']'])
                .enumerate()
                .map(|(i, s)| {
                    if i % 2 == 0 {
                        Ipv7seq::Supernet(s)
                    } else {
                        Ipv7seq::Hypernet(s)
                    }
                })
                .collect(),
        })
    }
}

impl Ipv7addr<'_> {
    fn tls_supported(&self) -> bool {
        fn contains_abba(s: &str) -> bool {
            for chunk in s.chars().collect::<Vec<char>>().windows(4) {
                if chunk[0] == chunk[3] && chunk[1] == chunk[2] && chunk[0] != chunk[1] {
                    return true;
                }
            }
            false
        }
        let mut found = false;
        for seq in &self.seqs {
            match seq {
                Ipv7seq::Supernet(s) => {
                    if contains_abba(s) {
                        found = true;
                    }
                }
                Ipv7seq::Hypernet(s) => {
                    if contains_abba(s) {
                        return false;
                    }
                }
            }
        }
        found
    }

    fn ssl_supported(&self) -> bool {
        let mut abas = HashSet::new();
        let mut babs = HashSet::new();
        for seq in &self.seqs {
            match seq {
                Ipv7seq::Supernet(s) => {
                    for chunk in s.chars().collect::<Vec<char>>().windows(3) {
                        if chunk[0] == chunk[2] && chunk[0] != chunk[1] {
                            abas.insert(chunk.iter().collect::<String>());
                        }
                    }
                }
                Ipv7seq::Hypernet(s) => {
                    for chunk in s.chars().collect::<Vec<char>>().windows(3) {
                        if chunk[0] == chunk[2] && chunk[0] != chunk[1] {
                            babs.insert([chunk[1], chunk[0], chunk[1]].iter().collect::<String>());
                        }
                    }
                }
            }
        }
        abas.intersection(&babs).count() > 0
    }
}

struct InputData<'a> {
    addresses: Vec<Ipv7addr<'a>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            addresses: s
                .lines()
                .map(|line| Ipv7addr::try_from(line).unwrap())
                .collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        self.addresses.iter().filter(|a| a.tls_supported()).count()
    }

    fn solve_part2(&self) -> usize {
        self.addresses.iter().filter(|a| a.ssl_supported()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "abba[mnop]qrst";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "abcd[bddb]xyyx";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "aaaa[qwer]tyui";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "ioxxoj[asdfgh]zxcvbn";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "aba[bab]xyz";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "xyx[xyx]xyx";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "aaa[kek]eke";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_4() {
        let testdata = "zazbz[bzb]cdb";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }
}
