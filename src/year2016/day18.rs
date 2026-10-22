//! # 2016 day 18 - Like a Rogue
//!
//! Represent the trap row as binary number where trap=1, safe=0. Then the state
//! for a tile in the next step will be the XOR of the tiles to the left and right.
//! So (tiles << 1) ^ (tiles >> 1), and then mask it with the original bitwidth so
//! it doesn't expand towards the left.
//!
//! Real input is 100 bits long, so we can use an u128 to represent its binary form.
//! Any input longer than 128 will cause the input parser to return AocError::Invalid.
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const PART_1_ROWS: u32 = 40;
const PART_2_ROWS: u32 = 400_000;

#[derive(Debug)]
struct InputData {
    start_row: u128,
    tile_width: u32,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile_width = s.len();
        if tile_width > 128 {
            Err(AocError::Invalid("input too long".to_string()))
        } else {
            let binary_input: String = s
                .chars()
                .map(|c| {
                    if c == '^' {
                        Ok('1')
                    } else if c == '.' {
                        Ok('0')
                    } else {
                        Err(AocError::Invalid(s.to_string()))
                    }
                })
                .collect::<Result<String, _>>()?;
            Ok(Self {
                start_row: u128::from_str_radix(&binary_input, 2)?,
                tile_width: s.len() as u32,
            })
        }
    }
}

impl InputData {
    fn get_nbr_safe_tiles(&self, rows: u32) -> u32 {
        let mut current = self.start_row;
        let mask: u128 = u128::MAX >> (128 - self.tile_width);
        let mut safe_tiles = self.tile_width - current.count_ones();
        for _ in 0..(rows - 1) {
            current = ((current << 1) ^ (current >> 1)) & mask;
            safe_tiles += self.tile_width - current.count_ones();
        }
        safe_tiles
    }

    fn solve_part1(&self) -> u32 {
        self.get_nbr_safe_tiles(PART_1_ROWS)
    }

    fn solve_part2(&self) -> u32 {
        self.get_nbr_safe_tiles(PART_2_ROWS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_safe_tiles_1() {
        let solution_data = InputData::from_str("..^^.").unwrap();
        assert_eq!(solution_data.get_nbr_safe_tiles(1), 3);
        assert_eq!(solution_data.get_nbr_safe_tiles(3), 6);
    }

    #[test]
    fn get_safe_tiles_2() {
        let solution_data = InputData::from_str(".^^.^.^^^^").unwrap();
        assert_eq!(solution_data.get_nbr_safe_tiles(10), 38);
    }

    #[test]
    fn get_safe_tiles_3() {
        let solution_data = InputData::from_str(".....").unwrap();
        assert_eq!(solution_data.get_nbr_safe_tiles(4), 20);
    }

    #[test]
    fn parser_illegal_char() {
        let err = InputData::from_str(".^^.^.^-^^").unwrap_err();
        assert_eq!(err, AocError::Invalid(".^^.^.^-^^".to_string()));
    }

    #[test]
    fn max_width_input() {
        // 128 columns is the widest that fits in a u128. The mask that keeps the
        // row from growing must cover the full width without overflowing the
        // shift: a 128 bit shift panics in debug builds and silently yields a
        // zero mask in release builds.
        let all_traps = "^".repeat(128);
        let solution_data = InputData::from_str(&all_traps).unwrap();
        assert_eq!(solution_data.tile_width, 128);
        assert_eq!(solution_data.get_nbr_safe_tiles(1), 0);
        assert_eq!(solution_data.get_nbr_safe_tiles(2), 126);
        assert_eq!(solution_data.get_nbr_safe_tiles(3), 252);
        assert_eq!(solution_data.get_nbr_safe_tiles(4), 376);

        let alternating = "^.^.".repeat(32);
        let solution_data = InputData::from_str(&alternating).unwrap();
        assert_eq!(solution_data.tile_width, 128);
        assert_eq!(solution_data.get_nbr_safe_tiles(1), 64);
        assert_eq!(solution_data.get_nbr_safe_tiles(2), 191);
        assert_eq!(solution_data.get_nbr_safe_tiles(3), 318);
        assert_eq!(solution_data.get_nbr_safe_tiles(4), 444);
    }

    #[test]
    fn parser_too_long() {
        // 129 columns is one too many for a u128. The content is irrelevant
        // here, only the width matters.
        let too_long = "^".repeat(129);
        let err = InputData::from_str(&too_long).unwrap_err();
        assert_eq!(err, AocError::Invalid("input too long".to_string()));
    }
}
