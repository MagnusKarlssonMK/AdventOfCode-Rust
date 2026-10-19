//! # 2016 day 14 - One-Time Pad
//!
//! Each candidate index is hashed once (`salt + index`); part 2 additionally
//! re-hashes the hex digest 2016 more times ("key stretching"). A hash is a
//! key if it contains a run of three equal characters and one of the next 1000
//! hashes contains five of that same character in a row.
//!
//! The per-index stretch chains are independent, so they are computed in
//! parallel in batches; the triple/quintuple matching that follows is
//! inherently sequential and cheap by comparison.
use crate::aoc_util::error::AocError;
use md5::{Digest, Md5};
use std::{collections::HashMap, error::Error, fmt::Write, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

/// Number of keys needed for the answer
const KEYS_NEEDED: usize = 64;
/// How many hashes after a triple may confirm it as a key.
const LOOKAHEAD: usize = 1000;
/// Extra md5 rounds applied to every digest in part 2.
const STRETCH: usize = 2016;
/// Hashes computed per batch before checking whether the answer is settled.
/// The real 64th key sits a little past 22000, so this finds it in one pass.
const BATCH: usize = 25_000;

/// Lowercase hex digits, indexed by nibble value. Used to encode a digest
/// without going through `std::fmt` (which dominated the stretch loop).
const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

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

impl InputData {
    fn solve_part1(&self) -> usize {
        self.solve(0)
    }

    fn solve_part2(&self) -> usize {
        self.solve(STRETCH)
    }

    /// Stretches hashes in growing batches until the 64th key is settled.
    fn solve(&self, stretch: usize) -> usize {
        let mut hashes: Vec<[u8; 32]> = Vec::new();
        let mut limit = BATCH;
        loop {
            self.stretch_hashes(&mut hashes, limit, stretch);
            if let Some(answer) = scan(&hashes) {
                return answer;
            }
            limit *= 2;
        }
    }

    /// Extends `hashes` with the hex digest of `salt + index` for every index
    /// below `limit`, applying `stretch` extra md5 rounds to each. Indices are
    /// independent, so the new range is split across the available cores.
    fn stretch_hashes(&self, hashes: &mut Vec<[u8; 32]>, limit: usize, stretch: usize) {
        let start = hashes.len();
        if start >= limit {
            return;
        }
        hashes.resize(limit, [0u8; 32]);

        let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
        let chunk = (limit - start).div_ceil(threads);

        std::thread::scope(|scope| {
            for (chunk_index, out) in hashes[start..].chunks_mut(chunk).enumerate() {
                let base = start + chunk_index * chunk;
                let salt = &self.salt;
                scope.spawn(move || {
                    let mut input = String::with_capacity(salt.len() + 20);
                    let mut hex = [0u8; 32];
                    let mut hasher = Md5::new();
                    for (offset, slot) in out.iter_mut().enumerate() {
                        input.clear();
                        input.push_str(salt);
                        write!(input, "{}", base + offset).unwrap();

                        hasher.update(input.as_bytes());
                        let mut digest = hasher.finalize_reset();
                        for _ in 0..stretch {
                            write_hex(&mut hex, &digest);
                            hasher.update(hex);
                            digest = hasher.finalize_reset();
                        }
                        write_hex(slot, &digest);
                    }
                });
            }
        });
    }
}

/// Scans the precomputed hashes in index order and returns the index of the
/// 64th key once it can no longer be displaced by a later find, or `None` if
/// the supplied range is too short.
fn scan(hashes: &[[u8; 32]]) -> Option<usize> {
    let mut triples: HashMap<u8, Vec<usize>> = HashMap::new();
    let mut keys: Vec<usize> = Vec::with_capacity(KEYS_NEEDED);

    for (index, hex) in hashes.iter().enumerate() {
        // Match five-in-a-row against pending triples. This must happen
        // before recording this hash's own triple: a hash containing
        // `aaaaa` also contains `aaa`, and must not confirm itself.
        for w in hex.windows(5) {
            if w.iter().all(|&b| b == w[0])
                && let Some(indices) = triples.remove(&w[0])
            {
                for i in indices {
                    if index - i <= LOOKAHEAD {
                        keys.push(i);
                    }
                }
            }
        }

        // Record the first three-in-a-row as a future candidate.
        if let Some(c) = first_run(hex, 3) {
            triples.entry(c).or_default().push(index);
        }

        // Keys are the *triple* indices and can be confirmed out of order, so
        // sort before taking the 64th. Once the 64th can no longer be displaced
        // by a later find - i.e. we're past its lookahead window - it is final.
        if keys.len() >= KEYS_NEEDED {
            keys.sort_unstable();
            let answer = keys[KEYS_NEEDED - 1];
            if answer + LOOKAHEAD < index + 1 {
                return Some(answer);
            }
        }
    }
    None
}

/// Writes the lowercase hex of `digest` into `out` (two bytes per digest byte).
fn write_hex(out: &mut [u8; 32], digest: &[u8]) {
    for (i, &byte) in digest.iter().enumerate() {
        out[2 * i] = HEX_CHARS[(byte >> 4) as usize];
        out[2 * i + 1] = HEX_CHARS[(byte & 0x0f) as usize];
    }
}

/// Returns the first byte that repeats `n` times consecutively in `hash`.
fn first_run(hash: &[u8], n: usize) -> Option<u8> {
    hash.windows(n)
        .find(|w| w.iter().all(|&b| b == w[0]))
        .map(|w| w[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str("abc").unwrap();
        assert_eq!(solution_data.solve_part1(), 22728);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str("abc").unwrap();
        assert_eq!(solution_data.solve_part2(), 22551);
    }
}
