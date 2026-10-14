//! # 2016 day 4 - Security Through Obscurity
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2?.to_string()))
}

#[derive(Debug)]
struct Room<'a> {
    name: Vec<&'a str>,
    sector_id: usize,
    checksum: &'a str,
}

impl<'a> TryFrom<&'a str> for Room<'a> {
    type Error = AocError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (name, id_checksum) = value.rsplit_once('-').ctx("sector-id and checksum")?;
        let (sector_id, checksum) = id_checksum.split_once('[').ctx("[")?;
        let checksum = checksum.strip_suffix(']').ctx("]")?;
        if name.is_empty() {
            Err(AocError::Missing("name"))
        } else {
            Ok(Self {
                name: name.split('-').collect(),
                sector_id: sector_id.parse()?,
                checksum,
            })
        }
    }
}

impl Room<'_> {
    fn validate(&self) -> Option<usize> {
        // Count downwards with negative numbers so that higher counts will get smaller values.
        // Thus easier to use sort later to get both count and character in correct order.
        let mut counts = HashMap::new();
        for word in &self.name {
            for c in word.chars() {
                counts.entry(c).and_modify(|e| *e -= 1).or_insert(-1);
            }
        }
        let mut sorted_counts: Vec<(isize, char)> = counts.iter().map(|(k, v)| (*v, *k)).collect();
        sorted_counts.sort();
        if sorted_counts.len() >= self.checksum.len() {
            let candidate: String = sorted_counts
                .iter()
                .take(self.checksum.len())
                .map(|(_, c)| *c)
                .collect();
            if self.checksum == candidate {
                return Some(self.sector_id);
            }
        }
        None
    }

    fn decode_check(&self, target: &[&str]) -> bool {
        // 1. Check number of words
        if self.name.len() != target.len() {
            return false;
        }
        // 2. Check number of letters in each word
        for (left, right) in self.name.iter().zip(target) {
            if left.len() != right.len() {
                return false;
            }
        }
        // 3. Decode the words and compare to target
        let d: Vec<String> = self
            .name
            .iter()
            .map(|word| {
                word.as_bytes()
                    .iter()
                    .map(|c| (b'a' + (*c + (self.sector_id % 26) as u8 - b'a') % 26) as char)
                    .collect()
            })
            .collect();
        d == target
    }
}

#[derive(Debug)]
struct InputData<'a> {
    rooms: Vec<Room<'a>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            rooms: value
                .lines()
                .map(Room::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (usize, Result<usize, AocError>) {
        let mut p1 = 0;
        let mut p2 = Err(AocError::NoSolution);
        const TARGET: [&str; 3] = ["northpole", "object", "storage"];
        for room in &self.rooms {
            if let Some(s_id) = room.validate() {
                p1 += s_id;
                if room.decode_check(&TARGET) {
                    p2 = Ok(s_id);
                }
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    const TEST_CUSTOM_1: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
lmprfnmjc-mzhcar-qrmpyec-548[mcrpa]";

    const TEST_CUSTOM_2: &str = "lmprfnmjc-mzhcar-qrmpyec-548[decoy]";

    const TEST_CUSTOM_3: &str = "abc-def-ghij-1[abcde]";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1514);
        assert_eq!(p2.unwrap_err(), AocError::NoSolution);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data = InputData::try_from(TEST_CUSTOM_1).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2.unwrap(), 548);
    }

    #[test]
    fn part2_custom_2() {
        // Same name and sector as the room in part2_custom_1, but the checksum fails, so it may not answer.
        let solution_data = InputData::try_from(TEST_CUSTOM_2).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2.unwrap_err(), AocError::NoSolution);
    }

    #[test]
    fn part2_custom_3() {
        // Valid checksum, but the decoded name is not the target room
        let solution_data = InputData::try_from(TEST_CUSTOM_3).unwrap();
        let (_, p2) = solution_data.solve();
        assert_eq!(p2.unwrap_err(), AocError::NoSolution);
    }

    #[test]
    fn parser_missing_left_bracket() {
        let err = InputData::try_from("abc-def-ghij-1abcde]").unwrap_err();
        assert_eq!(err, AocError::Missing("["));
    }

    #[test]
    fn parser_missing_right_bracket() {
        let err = InputData::try_from("abc-def-ghij-1[abcde").unwrap_err();
        assert_eq!(err, AocError::Missing("]"));
    }

    #[test]
    fn parser_missing_sectorid_checksum() {
        let err = InputData::try_from("abcdefghij1[abcde]").unwrap_err();
        assert_eq!(err, AocError::Missing("sector-id and checksum"));
    }

    #[test]
    fn parser_missing_name() {
        let err = InputData::try_from("-1[abcde]").unwrap_err();
        assert_eq!(err, AocError::Missing("name"));
    }

    #[test]
    fn parser_non_integer() {
        let err = InputData::try_from("abc-def-ghij-1r[abcde]").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }
}
