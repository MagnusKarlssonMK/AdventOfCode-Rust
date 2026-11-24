//! # 2024 day 9 - Disk Fragmenter
//!
//! ## Part 1
//!
//! The disk map alternates file lengths and free-space lengths. Simulate compaction by filling
//! target positions left to right: take blocks from the next file on the left, except while that
//! side is free space, in which case take blocks from the rightmost remaining file. Each placed
//! block contributes `target_position * file_id` to the checksum, where `file_id` is half the
//! disk-map index. Stop once the two sides meet or cross.
//!
//! ## Part 2
//!
//! Collect the free-space runs as `(start, length)` blocks in absolute disk positions. Walk file
//! runs right to left, tracking each run's start, and move the whole run into the earliest free
//! block that fits before it; trim the free block, or remove it when fully used. Checksum the
//! resulting occupied positions with the same `file_id` rule.
use crate::aoc_util::error::AocError;
use std::{collections::VecDeque, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct MemBlock {
    start: usize,
    length: usize,
}

struct InputData {
    disk_map: Vec<u8>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            disk_map: s
                .bytes()
                .map(|b| match b {
                    b'0'..=b'9' => Ok(b - b'0'),
                    _ => Err(AocError::Invalid("non-digit char in input".to_string())),
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        let mut left_idx = 0;
        let mut right_idx = self.disk_map.len() - 1;
        if right_idx % 2 == 1 {
            right_idx -= 1;
        } // Just in case input is even length
        let mut right_counter = self.disk_map[right_idx];
        let mut target_idx = 0usize;
        let mut checksum = 0;
        let mut buffer = self.disk_map[left_idx];
        while right_idx >= left_idx {
            if buffer > 0 {
                if left_idx % 2 == 0 {
                    buffer -= 1;
                    checksum += target_idx as u64 * left_idx as u64 / 2;
                    target_idx += 1;
                } else if right_counter > 0 {
                    buffer -= 1;
                    right_counter -= 1;
                    checksum += target_idx as u64 * right_idx as u64 / 2;
                    target_idx += 1;
                } else {
                    right_idx -= 2;
                    right_counter = self.disk_map[right_idx];
                }
            } else {
                left_idx += 1;
                if left_idx >= right_idx {
                    buffer = right_counter;
                } else {
                    buffer = self.disk_map[left_idx];
                }
            }
        }
        checksum
    }

    fn solve_part2(&self) -> u64 {
        let mut emptyblocks: VecDeque<MemBlock> =
            VecDeque::with_capacity(self.disk_map.len().div_ceil(2));
        let mut mempos = 0;
        for (i, v) in self.disk_map.iter().enumerate() {
            if i % 2 == 1 && *v > 0 {
                emptyblocks.push_back(MemBlock {
                    start: mempos,
                    length: *v as usize,
                });
            }
            mempos += *v as usize;
        }
        let mut checksum = 0;
        'outer: for (memid, memlen) in self.disk_map.iter().enumerate().rev() {
            let memlen = *memlen as usize;
            //Note: the memory id is actually half the index, so divide memid by 2 later when used
            mempos -= memlen;
            if memid % 2 == 0 {
                for eidx in 0..emptyblocks.len() {
                    if emptyblocks[eidx].start >= mempos {
                        break;
                    }
                    if emptyblocks[eidx].length >= memlen {
                        checksum += (emptyblocks[eidx].start..emptyblocks[eidx].start + memlen)
                            .map(|i| i as u64 * memid as u64 / 2)
                            .sum::<u64>();
                        if emptyblocks[eidx].length - memlen > 0 {
                            emptyblocks[eidx] = MemBlock {
                                start: emptyblocks[eidx].start + memlen,
                                length: emptyblocks[eidx].length - memlen,
                            };
                        } else {
                            emptyblocks.remove(eidx);
                        }
                        continue 'outer;
                    }
                }
                checksum += (mempos..mempos + memlen)
                    .map(|i| i as u64 * memid as u64 / 2)
                    .sum::<u64>();
            }
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "2333133121414131402";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1928);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 2858);
    }

    #[test]
    fn part1_custom_1() {
        // File lengths 0
        let solution_data: InputData = "0".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        let solution_data: InputData = "0112120".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
        let solution_data: InputData = "02110320".parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 5);
    }
}
