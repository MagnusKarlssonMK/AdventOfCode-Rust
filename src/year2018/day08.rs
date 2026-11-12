//! # 2018 day 8 - Memory Maneuver
//!
//! Tree encoded as a flat array: each node starts with `(child_count, meta_count)`
//! followed by the serialized children, followed by `meta_count` metadata entries.
//! Part 1 sums all metadata entries; part 2 gives the root value where leaves sum
//! their metadata and internal nodes sum the values of children referenced by
//! their metadata (metadata entries are 1-indexed).
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug)]
struct InputData {
    numbers: Vec<usize>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .split_ascii_whitespace()
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    /// Decode a node starting at `idx`. Returns `(next_idx, metadata_sum, node_value)`,
    /// where `next_idx` is the index immediately after this node in the flattened array.
    fn decode(&self, idx: usize) -> Result<(usize, usize, usize), AocError> {
        let child_count = *self.numbers.get(idx).ctx("node header: child count")?;
        let meta_count = *self
            .numbers
            .get(idx + 1)
            .ctx("node header: metadata count")?;
        let mut cursor = idx + 2;

        // Recursively decode all children first; their metadata sums accumulate into p1.
        let mut metadata_sum_total: usize = 0;
        let mut child_values = Vec::with_capacity(child_count);
        for _ in 0..child_count {
            let (next_cursor, child_metadata_sum, child_value) = self.decode(cursor)?;
            cursor = next_cursor;
            metadata_sum_total += child_metadata_sum;
            child_values.push(child_value);
        }

        // Read this node's metadata entries.
        let metadata = self
            .numbers
            .get(cursor..cursor + meta_count)
            .ctx("node metadata")?;
        let metadata_sum = metadata.iter().sum::<usize>();

        // Compute node value: leaves sum metadata; internal nodes sum values of children
        // referenced by metadata entries (1-indexed).
        let node_value = if child_count == 0 {
            metadata_sum
        } else {
            let child_count_usize = child_values.len();
            metadata
                .iter()
                .filter(|&&m| m >= 1 && m <= child_count_usize)
                .map(|&m| child_values[m - 1])
                .sum()
        };

        Ok((
            cursor + meta_count,
            metadata_sum_total + metadata_sum,
            node_value,
        ))
    }

    fn solve(&self) -> Result<(usize, usize), AocError> {
        let (_, metadata_sum_p1, root_value_p2) = self.decode(0)?;
        Ok((metadata_sum_p1, root_value_p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    /// The root's only metadata value (2) references a child that doesn't exist.
    const TEST_CUSTOM_1: &str = "1 1 0 1 5 2";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 138);
        assert_eq!(p2, 66);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 7);
        assert_eq!(p2, 0);
    }

    #[test]
    fn decode_errors_invalid_input() {
        // Missing metadata count/header
        let solution_data = InputData::from_str("2").unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("node header: metadata count".to_string()));

        // Header ok but missing children/metadata
        let solution_data = InputData::from_str("1 1").unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("node header: child count".to_string()));

        // Child exists but metadata slice is truncated
        let solution_data = InputData::from_str("1 2 0 1").unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::Invalid("node metadata".to_string()));
    }
}
