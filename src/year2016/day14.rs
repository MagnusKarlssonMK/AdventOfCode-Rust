//! # 2016 day 14 - One-Time Pad
//!
//! Each candidate index is hashed once (`salt + index`); part 2 additionally
//! re-hashes the hex digest 2016 more times ("key stretching"). A hash is a
//! key if it contains a run of three equal characters and one of the next 1000
//! hashes contains five of that same character in a row.
use crate::aoc_util::error::AocError;
use md5::{Digest, Md5};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Write,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const KEYS_NEEDED: usize = 64;
/// How many hashes after a triple may confirm it as a key.
const LOOKAHEAD: usize = 1000;
/// Extra md5 rounds applied to every digest in part 2.
const STRETCH: usize = 2016;

struct InputData {
    salt: String,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            salt: s.trim().to_string(),
        })
    }
}

/// Returns the first byte that repeats `n` times consecutively in `hash`.
fn first_run(hash: &[u8], n: usize) -> Option<u8> {
    hash.windows(n)
        .find(|w| w.iter().all(|&b| b == w[0]))
        .map(|w| w[0])
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.solve(0)
    }

    fn solve_part2(&self) -> usize {
        self.solve(STRETCH)
    }

    /// Lowercase hex digest of `salt + index`, stretched by `stretch` extra
    /// md5 rounds (each round hashes the previous round's hex string).
    fn hash_hex(&self, index: usize, stretch: usize) -> String {
        let mut buffer = format!("{}{index}", self.salt);
        let mut digest = Md5::digest(buffer.as_bytes());
        for _ in 0..stretch {
            buffer.clear();
            for byte in digest.iter() {
                write!(buffer, "{byte:02x}").unwrap();
            }
            digest = Md5::digest(buffer.as_bytes());
        }
        let mut hex = String::with_capacity(32);
        for byte in digest.iter() {
            write!(hex, "{byte:02x}").unwrap();
        }
        hex
    }

    /// Scans hashes in increasing index order, collecting the indices of the
    /// hashes that qualify as keys.
    fn solve(&self, stretch: usize) -> usize {
        let mut triples: HashMap<u8, Vec<usize>> = HashMap::new();
        let mut keys: Vec<usize> = Vec::with_capacity(KEYS_NEEDED);
        let mut index = 0usize;

        loop {
            let hexdigest = self.hash_hex(index, stretch);
            let bytes = hexdigest.as_bytes();

            // Match five-in-a-row against pending triples. This must happen
            // before recording this hash's own triple: a hash containing
            // `aaaaa` also contains `aaa`, and must not confirm itself.
            let quintuple_chars: HashSet<u8> = bytes
                .windows(5)
                .filter(|w| w.iter().all(|&b| b == w[0]))
                .map(|w| w[0])
                .collect();
            for c in quintuple_chars {
                if let Some(indices) = triples.remove(&c) {
                    for i in indices {
                        if index - i <= LOOKAHEAD {
                            keys.push(i);
                        }
                    }
                }
            }

            // Record the first three-in-a-row as a future candidate.
            if let Some(c) = first_run(bytes, 3) {
                triples.entry(c).or_default().push(index);
            }

            index += 1;

            // Keys are the *triple* indices and can be confirmed out of order,
            // so sort before taking the 64th. Once the 64th can no longer be
            // displaced by a later find - i.e. we're past its lookahead window
            // - it is final.
            if keys.len() >= KEYS_NEEDED {
                keys.sort_unstable();
                let answer = keys[KEYS_NEEDED - 1];
                if answer + LOOKAHEAD < index {
                    return answer;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let testdata = "abc";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 22728);
    }

    #[test]
    #[ignore = "slow"]
    fn part2_official_1() {
        let testdata = "abc";
        let solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 22551);
    }
}
