//! # 2016 day 4 - Security Through Obscurity
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct Room<'a> {
    name: Vec<&'a str>,
    sector_id: usize,
    checksum: &'a str,
}

impl<'a> TryFrom<&'a str> for Room<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut name = Vec::new();
        let mut sector_id = 0;
        let mut checksum = "";
        for part in value.split('-') {
            if part.ends_with(']') {
                let (left, right) = part.split_once('[').unwrap();
                sector_id = left.parse().unwrap();
                checksum = right.trim_end_matches(']');
            } else {
                name.push(part);
            }
        }
        Ok(Self {
            name,
            sector_id,
            checksum,
        })
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

struct InputData<'a> {
    rooms: Vec<Room<'a>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            rooms: value
                .lines()
                .map(|line| Room::try_from(line).unwrap())
                .collect(),
        })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        const TARGET: [&str; 3] = ["northpole", "object", "storage"];
        for room in &self.rooms {
            if let Some(s_id) = room.validate() {
                p1 += s_id;
                if room.decode_check(&TARGET) {
                    p2 = s_id;
                }
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
        let testdata = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 1514);
    }
}
