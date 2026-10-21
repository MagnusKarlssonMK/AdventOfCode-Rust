//! # 2016 day 16 - Dragon Checksum
//!
//! A naive solution materialises up to 35 million dragon-curve bits and folds
//! them repeatedly. To avoid that, we instead make use of two properties to get
//! both answers almost instantly.
//!
//! ## 1. The checksum is a per-block parity
//!
//! One checksum round maps each adjacent pair to `1` if the bits are equal
//! and `0` if they differ (XNOR). XNOR-reducing a block of `2^k` bits yields
//! `1` exactly when the block has an even number of ones, so after the
//! checksum has been halved `m` times each remaining bit is the parity of a
//! `2^m`-bit block. Both disk sizes are `odd * 2^m`, so `m` is the number of
//! trailing zeros and the block size is the lowest set bit.
//!
//! ## 2. Prefix popcounts come from the dragon curve's self-similarity
//!
//! With `n = input.len()` and dragon levels `L_k = 2^k * (n + 1) - 1`, level
//! `k` is `D_k = D_{k-1} + '0' + complement(reverse(D_{k-1}))`. For a prefix
//! length `L` in `(L_{k-1}, L_k]`, the ones in the spilled-over flipped tail
//! cancel against the ones at the front of `D_{k-1}`, leaving
//!
//! ```text
//! ones(L) = (L - L_{k-1} - 1) + ones(L_k - L)
//! ```
//!
//! That recursion shrinks `L` by one dragon level per step, and
//! `L_k / 2 == L_{k-1}` under integer division, so the code simply halves its
//! level lengths. Once the remaining prefix fits inside the input, a
//! prefix-sum lookup gives the answer.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((solution_data.solve_part1(), solution_data.solve_part2()))
}

const DISK_SIZE_PART_1: usize = 272;
const DISK_SIZE_PART_2: usize = 35_651_584;

/// Number of ones in the first `length` bits of the infinite dragon-curve data.
///
/// `prefix_ones[i]` is the number of ones in the first `i` bits of the original
/// input, which is the base case once `length` is small enough.
fn count_ones(prefix_ones: &[usize], mut length: usize) -> usize {
    // Lengths of consecutive dragon levels: initially L_0 and L_1. After the
    // loop, `level_len` is the smallest L_k >= length and `prev_len` is L_{k-1}.
    let mut prev_len = prefix_ones.len() - 1;
    let mut level_len = 2 * prev_len + 1;
    while level_len < length {
        prev_len = level_len;
        level_len = 2 * prev_len + 1;
    }

    let mut result = 0;

    // Peel off one dragon level per iteration. `length` strictly decreases and
    // the loop stops as soon as the remaining prefix fits inside the input.
    while length >= prefix_ones.len() {
        // Descend to the level with prev_len < length <= level_len. Halving
        // works because L_k / 2 == L_{k-1} under integer division.
        while length <= prev_len {
            prev_len /= 2;
            level_len /= 2;
        }

        // `remaining = L_k - length`, and `t = L - L_{k-1} - 1` bits are taken
        // from the flipped tail, so `t == prev_len - remaining`. Those tail
        // ones cancel the whole-level ones, leaving the recursion
        // `ones(L) = t + ones(remaining)`.
        let remaining = level_len - length;
        result += prev_len - remaining;
        length = remaining;
    }

    result + prefix_ones[length]
}

#[derive(Debug)]
struct InputData {
    /// `prefix_ones[i]` is the number of ones in the first `i` bits of the
    /// input, so the popcount of any input prefix is an O(1) lookup.
    prefix_ones: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(AocError::Invalid(s.to_string()));
        }

        let mut prefix_ones = Vec::with_capacity(s.len() + 1);
        prefix_ones.push(0);
        for c in s.chars() {
            let ones = *prefix_ones.last().unwrap();
            match c {
                '1' => prefix_ones.push(ones + 1),
                '0' => prefix_ones.push(ones),
                _ => return Err(AocError::Invalid(s.to_string())),
            }
        }

        Ok(Self { prefix_ones })
    }
}

impl InputData {
    fn get_checksum(&self, disk_size: usize) -> String {
        // After `m` checksum halvings each output bit is the parity of a
        // 2^m-bit block; since disk_size is odd * 2^m, that block size is the
        // lowest set bit (2^m).
        let block_size = disk_size.isolate_lowest_one();
        let number_of_blocks = disk_size / block_size;

        // `counts[i]` is the number of ones in the first `i` blocks, so the
        // difference between neighbours is the popcount of one whole block.
        let counts: Vec<_> = (0..=number_of_blocks)
            .map(|i| count_ones(&self.prefix_ones, i * block_size))
            .collect();

        counts
            .array_windows()
            .map(
                |&[before, after]| {
                    if (after - before) % 2 == 0 { '1' } else { '0' }
                },
            )
            .collect()
    }

    fn solve_part1(&self) -> String {
        self.get_checksum(DISK_SIZE_PART_1)
    }

    fn solve_part2(&self) -> String {
        self.get_checksum(DISK_SIZE_PART_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_checksum_1() {
        let solution_data = InputData::from_str("110010110100").unwrap();
        assert_eq!(solution_data.get_checksum(12), "100");
    }

    #[test]
    fn get_checksum_2() {
        let solution_data = InputData::from_str("10000").unwrap();
        assert_eq!(solution_data.get_checksum(20), "01100");
    }

    /// Builds the first `length` bits of the dragon-curve data the naive way.
    fn naive_data(initial: &str, length: usize) -> Vec<u8> {
        let mut data: Vec<u8> = initial.bytes().collect();
        while data.len() < length {
            let mut next = Vec::with_capacity(2 * data.len() + 1);
            next.extend_from_slice(&data);
            next.push(b'0');
            next.extend(
                data.iter()
                    .rev()
                    .map(|b| if *b == b'1' { b'0' } else { b'1' }),
            );
            data = next;
        }
        data.truncate(length);
        data
    }

    #[test]
    fn count_ones_matches_naive_dragon_data() {
        for initial in ["1", "10000", "110010110100"] {
            let data = naive_data(initial, 200);
            let solution_data = InputData::from_str(initial).unwrap();
            let mut expected = 0;
            for (i, bit) in data.iter().enumerate() {
                expected += usize::from(*bit == b'1');
                assert_eq!(
                    count_ones(&solution_data.prefix_ones, i + 1),
                    expected,
                    "input {initial}, prefix length {}",
                    i + 1
                );
            }
        }
    }

    #[test]
    fn parser_invalid_char() {
        let bad_input = "01d10";
        let err = InputData::from_str(bad_input).unwrap_err();
        assert_eq!(err, AocError::Invalid(bad_input.to_string()));
    }

    #[test]
    fn parser_empty_input() {
        let err = InputData::from_str("").unwrap_err();
        assert_eq!(err, AocError::Invalid(String::new()));
    }
}
