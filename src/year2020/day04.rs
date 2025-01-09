//! # 2020 day 4 - Passport Processing

use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl FromStr for Field {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Self::Byr),
            "iyr" => Ok(Self::Iyr),
            "eyr" => Ok(Self::Eyr),
            "hgt" => Ok(Self::Hgt),
            "hcl" => Ok(Self::Hcl),
            "ecl" => Ok(Self::Ecl),
            "pid" => Ok(Self::Pid),
            "cid" => Ok(Self::Cid),
            _ => Err(()),
        }
    }
}

impl Field {
    fn is_valid(&self, value: &str) -> bool {
        match self {
            Self::Byr => {
                if let Ok(v) = value.parse::<usize>() {
                    if !(1920..=2002).contains(&v) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Self::Iyr => {
                if let Ok(v) = value.parse::<usize>() {
                    if !(2010..=2020).contains(&v) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Self::Eyr => {
                if let Ok(v) = value.parse::<usize>() {
                    if !(2020..=2030).contains(&v) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Self::Hgt => {
                let (left, right) = value.split_at(value.len() - 2);
                if let Ok(n) = left.parse::<usize>() {
                    match right {
                        "cm" => {
                            if !(150..=193).contains(&n) {
                                return false;
                            }
                        }
                        "in" => {
                            if !(59..=76).contains(&n) {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
            Self::Hcl => {
                if let Some(v) = value.strip_prefix('#') {
                    if !v.chars().all(|c| "abcdef0123456789".contains(c)) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Self::Ecl => {
                if !&["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .contains(&value)
                {
                    return false;
                }
            }
            Self::Pid => {
                if value.len() != 9 || value.parse::<usize>().is_err() {
                    return false;
                }
            }
            Self::Cid => (),
        }
        true
    }
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<Field, String>,
}

impl FromStr for Passport {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            fields: s
                .split_whitespace()
                .map(|f| {
                    let (left, right) = f.split_once(':').unwrap();
                    (Field::from_str(left).unwrap(), right.to_string())
                })
                .collect(),
        })
    }
}

enum Validity {
    Relaxed,
    Strict,
    Invalid,
}

impl Passport {
    fn validity(&self) -> Validity {
        if self.fields.len() == 8
            || (self.fields.len() == 7 && !self.fields.contains_key(&Field::Cid))
        {
            for (field, value) in &self.fields {
                if !field.is_valid(value) {
                    return Validity::Relaxed;
                }
            }
            return Validity::Strict;
        }
        Validity::Invalid
    }
}

struct InputData {
    passports: Vec<Passport>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            passports: input
                .split("\n\n")
                .map(|line| Passport::from_str(line).unwrap())
                .collect(),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        for p in &self.passports {
            match p.validity() {
                Validity::Relaxed => {
                    p1 += 1;
                }
                Validity::Strict => {
                    p1 += 1;
                    p2 += 1;
                }
                Validity::Invalid => (),
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 2);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        let solution_data = InputData::parse_input(&testdata);
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 4);
    }
}
