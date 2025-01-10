//! # 2024 day 9 - Disk Fragmenter
//!
//! ## Part 1
//!
//! Some index juggling...
//!
//! ## Part 2
//!
//! Some slightly different index juggling...
use std::{collections::VecDeque, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
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
    disk_map: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            disk_map: s
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut left_idx = 0;
        let mut right_idx = self.disk_map.len() - 1;
        if right_idx % 2 == 1 {
            right_idx -= 1;
        } // Just in case input is even length
        let mut right_counter = self.disk_map[right_idx];
        let mut target_idx = 0;
        let mut checksum = 0;
        let mut buffer = self.disk_map[left_idx];
        while right_idx >= left_idx {
            if buffer > 0 {
                if left_idx % 2 == 0 {
                    buffer -= 1;
                    checksum += target_idx * left_idx / 2;
                    target_idx += 1;
                } else if right_counter > 0 {
                    buffer -= 1;
                    right_counter -= 1;
                    checksum += target_idx * right_idx / 2;
                    target_idx += 1;
                } else {
                    right_idx -= 2;
                    right_counter = self.disk_map[right_idx];
                }
            } else {
                left_idx += 1;
                if left_idx == right_idx {
                    buffer = right_counter;
                } else {
                    buffer = self.disk_map[left_idx];
                }
            }
        }
        checksum
    }

    fn solve_part2(&self) -> usize {
        let mut emptyblocks: VecDeque<MemBlock> =
            VecDeque::with_capacity((1 + self.disk_map.len()) / 2);
        let mut mempos = 0;
        for (i, v) in self.disk_map.iter().enumerate() {
            if i % 2 == 1 && *v > 0 {
                emptyblocks.push_back(MemBlock {
                    start: mempos,
                    length: *v,
                });
            }
            mempos += v;
        }
        let mut checksum = 0;
        'outer: for (memid, memlen) in self.disk_map.iter().enumerate().rev() {
            //Note: the memory id is actually half the index, so divide memid by 2 later when used
            mempos -= memlen;
            if memid % 2 == 0 {
                for eidx in 0..emptyblocks.len() {
                    if emptyblocks[eidx].start >= mempos {
                        break;
                    }
                    if emptyblocks[eidx].length >= *memlen {
                        checksum += (emptyblocks[eidx].start..emptyblocks[eidx].start + *memlen)
                            .map(|i| i * memid / 2)
                            .sum::<usize>();
                        if emptyblocks[eidx].length - memlen > 0 {
                            emptyblocks[eidx] = MemBlock {
                                start: emptyblocks[eidx].start + *memlen,
                                length: emptyblocks[eidx].length - memlen,
                            };
                        } else {
                            emptyblocks.remove(eidx);
                        }
                        continue 'outer;
                    }
                }
                checksum += (mempos..mempos + *memlen)
                    .map(|i| i * memid / 2)
                    .sum::<usize>();
            }
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2333133121414131402";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 1928);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 2858);
    }
}
