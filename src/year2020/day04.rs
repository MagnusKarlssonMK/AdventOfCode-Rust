//! # 2020 day 4 - Passport Processing
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
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
    type Err = AocError;
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
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Field {
    fn is_valid(&self, value: &str) -> bool {
        match self {
            Self::Byr => {
                if let Ok(v) = value.parse::<usize>() {
                    (1920..=2002).contains(&v)
                } else {
                    false
                }
            }
            Self::Iyr => {
                if let Ok(v) = value.parse::<usize>() {
                    (2010..=2020).contains(&v)
                } else {
                    false
                }
            }
            Self::Eyr => {
                if let Ok(v) = value.parse::<usize>() {
                    (2020..=2030).contains(&v)
                } else {
                    false
                }
            }
            Self::Hgt => {
                let (left, right) = value.split_at(value.len() - 2);
                if let Ok(n) = left.parse::<usize>() {
                    match right {
                        "cm" => (150..=193).contains(&n),
                        "in" => (59..=76).contains(&n),
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Self::Hcl => {
                if let Some(v) = value.strip_prefix('#') {
                    v.len() == 6 && v.chars().all(|c| "abcdef0123456789".contains(c))
                } else {
                    false
                }
            }
            Self::Ecl => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            Self::Pid => value.len() == 9 && value.parse::<usize>().is_ok(),
            Self::Cid => true,
        }
    }
}

struct Passport {
    fields: HashMap<Field, String>,
}

impl FromStr for Passport {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_field(s: &str) -> Result<(Field, String), AocError> {
            let (left, right) = s.split_once(':').ctx("colon separator")?;
            Ok((left.parse()?, right.to_string()))
        }
        Ok(Self {
            fields: s
                .split_whitespace()
                .map(parse_field)
                .collect::<Result<_, _>>()?,
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

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            passports: s
                .split("\n\n")
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
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

    const TEST_OFFICIAL_1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const TEST_OFFICIAL_2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const TEST_OFFICIAL_3: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    const TEST_CUSTOM_1: &str = "pid:087499704 hgt:74 ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";

    /// The hex colour must be exactly '#rrggbb' (six hex digits).
    const TEST_CUSTOM_2: &str =
        "byr:1980 iyr:2012 eyr:2030 hgt:180cm hcl:#abc ecl:grn pid:000000001

byr:1980 iyr:2012 eyr:2030 hgt:180cm hcl:#aabbcc ecl:grn pid:000000001";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 2);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 4);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 0);
    }

    #[test]
    fn part2_custom_2() {
        let solution_data: InputData = TEST_CUSTOM_2.parse().unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2, 1);
    }
}
