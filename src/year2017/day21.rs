//! # 2017 day 21 - Fractal Art
//!
//! First, pre-compute all permutations of the left-hand side of the rules and map to
//! the right-hand side in a HashMap, to avoid having to do the permutations later in
//! the enhancement process. Use binary representation of the patterns, e.g.
//!
//! * `.#/##` => `1,3`
//! * `.#./..#/###` => `2,1,7`
//!
//! The enhancement process consists of multiple steps:
//! * Break up the pattern in smaller parts of 2- or 3-sized patterns.
//! * Use the input rules to enhance each part.
//! * Re-combine the parts to a new image, unless the new image would be a 9-sized
//!   image, in which case it consists of 9 3-sized images that are now independent
//!   from each other in further enhancement steps. I.e. we can store our image state
//!   as a counter of different sub-patterns.
//!
//! So, start with 1 of the starting 3-size pattern => enhance it to one 4-size
//! pattern => break it up into 4 2-sized patterns, enhance and recombined into a
//! 6-size pattern => break up into 9 2-size patterns, enhance to 9 3-size patterns,
//! and instead of recombining; => start over with enhancing each unique shape and
//! keep track of the counter.
use crate::aoc_util::error::*;
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const PART1_ITERATIONS: usize = 5;
const PART2_ITERATIONS: usize = 18;
const START_IMAGE: &str = ".#./..#/###";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pattern {
    rows: Vec<u32>,
}

impl FromStr for Pattern {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for row in s.split('/') {
            let mut v = 0;
            for c in row.chars() {
                v <<= 1;
                if c == '#' {
                    v += 1;
                }
            }
            rows.push(v);
        }
        Ok(Self { rows })
    }
}

impl Pattern {
    /// Generates the rotated image of a pattern
    fn get_right_rotation(&self) -> Self {
        let size = self.rows.len();
        let mut rows = vec![0u32; size];
        for col in 0..size {
            for (i, row) in self.rows.iter().enumerate() {
                if (row >> col) & 1 > 0 {
                    rows[size - 1 - col] += 1 << i;
                }
            }
        }
        Self { rows }
    }

    /// Generates the mirror image of a pattern
    fn get_mirror(&self) -> Self {
        let size = self.rows.len();
        let mut rows = Vec::new();
        for row in self.rows.iter() {
            let mut new_row = 0;
            for i in 0..size {
                new_row <<= 1;
                if (row >> i) & 1 > 0 {
                    new_row += 1;
                }
            }
            rows.push(new_row);
        }
        Self { rows }
    }

    /// Generates and returns all permutations of a pattern, including the start pattern itself
    fn get_permutations(&self) -> Vec<Self> {
        let mut p1 = self.clone();
        let mut p2 = p1.get_mirror();
        let mut res = Vec::from([self.clone(), p2.clone()]);
        for _ in 0..3 {
            p1 = p1.get_right_rotation();
            p2 = p2.get_right_rotation();
            res.extend([p1.clone(), p2.clone()]);
        }
        res
    }

    /// Breaks the pattern up into smaller 2-size or 3-size parts, depending on its size. If the
    /// pattern is already a single 2-size or 3-size pattern, the returned Vec will contain only
    /// that same pattern. If the patterns size is a multiple of both 2 and 3 (e.g. size 6), an
    /// output size of 2 takes precedence.
    fn break_up(&self) -> Vec<Self> {
        let size = self.rows.len();
        let block_size = if size.is_multiple_of(2) { 2 } else { 3 };
        let blocks_per_axis = size / block_size;
        let mask = (1 << block_size) - 1;

        let mut res = Vec::with_capacity(blocks_per_axis * blocks_per_axis);
        for by in 0..blocks_per_axis {
            for bx in 0..blocks_per_axis {
                let shift = bx * block_size;
                let rows = (0..block_size)
                    .map(|dy| (self.rows[by * block_size + dy] >> shift) & mask)
                    .collect();
                res.push(Self { rows });
            }
        }
        res
    }

    /// Takes an input slice of patterns and re-combines them into a single pattern, using the same
    /// block order as they were previously split by `break_up()`.
    fn recombine(v: &[Self]) -> Self {
        let blocks_per_axis = v.len().isqrt();
        let block_size = v.first().unwrap().rows.len();
        let mut new_pattern = vec![0; blocks_per_axis * block_size];
        for (i, p) in v.iter().enumerate() {
            let bx = i % blocks_per_axis;
            let by = i / blocks_per_axis;
            for (dy, row) in p.rows.iter().enumerate() {
                new_pattern[(by * block_size) + dy] += row << (bx * block_size);
            }
        }
        Self { rows: new_pattern }
    }

    /// Enhances the pattern according to the rules and returns the result. If the result is a 9-size
    /// pattern it will be split up and returned as 9 individual 3-sized patterns, otherwise a single
    /// pattern will be returned.
    /// Returns AocError::NoSolution if the input rules lacks a necessary key for the enhancement.
    fn enhance(&self, rules: &HashMap<Pattern, Pattern>) -> Result<Vec<Self>, AocError> {
        // Step 1-2: Break the pattern up into smaller parts, and use the rules to enhance each part
        let parts: Vec<_> = self
            .break_up()
            .iter()
            .map(|p| rules.get(p).cloned().ok_or(AocError::NoSolution))
            .collect::<Result<Vec<_>, _>>()?;

        if parts.len() == 4 {
            // Four 3-sized blocks to be recombined into a 6-sized image
            // We can recombine and then split it up again into 9 2-sized patterns before returning;
            // these are equivalent with regard to number of 'on' cells in current state, and will
            // turn into 9 independent 3-size patterns in the next enhancement step. I.e. they will
            // already now be independent.
            let size6 = Self::recombine(&parts);
            Ok(size6.break_up())
        } else {
            // A single enhanced image, or nine independent images
            Ok(parts)
        }
    }

    /// Calculates how many pixels in the pattern that are on
    fn get_count(&self) -> u32 {
        self.rows.iter().map(|row| row.count_ones()).sum()
    }
}

struct InputData {
    enhancement_rules: HashMap<Pattern, Pattern>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut enhancement_rules = HashMap::new();
        for line in s.lines() {
            let (left, right) = line.split_once(" => ").ctx("=> separator")?;
            if left.len() > 11 {
                // We will only ever use enhancement mapping for patterns that have been broken down to 2- or
                // 3-size elements (9 characters plus two /-separators), so we can throw away all rules for
                // larger elements.
                continue;
            }
            let from_patterns = left.parse::<Pattern>()?.get_permutations();
            for p in &from_patterns {
                enhancement_rules.insert(p.clone(), right.parse()?);
            }
        }
        Ok(Self { enhancement_rules })
    }
}

impl InputData {
    fn enhance_and_count(&self, rounds: usize) -> Result<u32, AocError> {
        let mut patterns: HashMap<Pattern, u32> =
            HashMap::from([(START_IMAGE.parse::<Pattern>()?, 1)]);
        for _ in 0..rounds {
            let mut new_patterns = HashMap::new();
            for (pattern, count) in patterns.iter() {
                for p in pattern.enhance(&self.enhancement_rules)? {
                    new_patterns
                        .entry(p)
                        .and_modify(|v| *v += *count)
                        .or_insert(*count);
                }
            }
            patterns = new_patterns;
        }
        Ok(patterns.iter().map(|(k, v)| v * k.get_count()).sum())
    }

    fn solve_part1(&self) -> Result<u32, AocError> {
        self.enhance_and_count(PART1_ITERATIONS)
    }

    fn solve_part2(&self) -> Result<u32, AocError> {
        self.enhance_and_count(PART2_ITERATIONS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.enhance_and_count(1).unwrap(), 4);
        assert_eq!(solution_data.enhance_and_count(2).unwrap(), 12);
    }

    #[test]
    fn pattern_parser() {
        let p: Pattern = START_IMAGE.parse().unwrap();
        assert_eq!(p.rows, [2, 1, 7]);
        let p: Pattern = "..##/#.../.#../####".parse().unwrap();
        assert_eq!(p.rows, [3, 8, 4, 15]);
    }

    #[test]
    fn pattern_mirror() {
        let p: Pattern = START_IMAGE.parse().unwrap();
        assert_eq!(p.get_mirror().rows, [2, 4, 7]);
        let p: Pattern = "..##/#.../.#../####".parse().unwrap();
        assert_eq!(p.get_mirror().rows, [12, 1, 2, 15]);
    }

    #[test]
    fn pattern_rotate() {
        let p: Pattern = START_IMAGE.parse().unwrap();
        assert_eq!(p.get_right_rotation().rows, [4, 5, 6]);
        let p: Pattern = "..##/#.../.#../####".parse().unwrap();
        assert_eq!(p.get_right_rotation().rows, [10, 12, 9, 9]);
        let size2: Pattern = "../.#".parse().unwrap();
        let size2 = size2.get_right_rotation();
        assert_eq!(size2.rows, [0, 2]);
        let size2 = size2.get_right_rotation();
        assert_eq!(size2.rows, [2, 0]);
        let size2 = size2.get_right_rotation();
        assert_eq!(size2.rows, [1, 0]);
        let size2 = size2.get_right_rotation();
        assert_eq!(size2.rows, [0, 1]);
    }

    #[test]
    fn pattern_count() {
        let p: Pattern = START_IMAGE.parse().unwrap();
        assert_eq!(p.get_count(), 5);
        let p: Pattern = "..##/#.../.#../####".parse().unwrap();
        assert_eq!(p.get_count(), 8);
    }

    #[test]
    fn pattern_break_up() {
        // A single 3-size pattern
        let size3: Pattern = START_IMAGE.parse().unwrap();
        assert_eq!(size3.break_up(), vec![size3.clone()]);
        // Simple symmetrical case that breaks up into 4 identical smaller parts
        let size2: Pattern = ".#/##".parse().unwrap();
        assert_eq!(size2.break_up(), vec![size2.clone()]);
        let size4: Pattern = ".#.#/####/.#.#/####".parse().unwrap();
        assert_eq!(
            size4.break_up(),
            vec![size2.clone(), size2.clone(), size2.clone(), size2.clone()]
        );
        // Asymmetrical case with a 6-size breaking up into 9 2-size parts
        let size6: Pattern = "#...#./.###../..#.../##.##./...#.#/###.##".parse().unwrap();
        let a: Pattern = "#./..".parse().unwrap();
        let b: Pattern = "../##".parse().unwrap();
        let c: Pattern = "#./.#".parse().unwrap();
        let d: Pattern = "../#.".parse().unwrap();
        let e: Pattern = ".#/##".parse().unwrap();
        let f: Pattern = ".#/#.".parse().unwrap();
        let res = size6.break_up();
        assert_eq!(res, [a, b.clone(), c.clone(), d, c, b.clone(), e, f, b]);
    }

    #[test]
    fn pattern_recombine() {
        // Simple symmetrical case of 4 identical parts
        let size2: Pattern = ".#/##".parse().unwrap();
        let size4: Pattern = ".#.#/####/.#.#/####".parse().unwrap();
        let parts = [size2.clone(), size2.clone(), size2.clone(), size2.clone()];
        assert_eq!(Pattern::recombine(&parts), size4);
        // Asymmetrical case of 4 different 2-sized parts
        let size2_1: Pattern = "##/.#".parse().unwrap();
        let size2_2: Pattern = ".#/#.".parse().unwrap();
        let size2_3: Pattern = "##/#.".parse().unwrap();
        let size2_4: Pattern = "../##".parse().unwrap();
        let size4: Pattern = ".###/#..#/..##/###.".parse().unwrap();
        let parts = [size2_1, size2_2, size2_3, size2_4];
        assert_eq!(Pattern::recombine(&parts), size4);
        // Asymmetrical case of 4 different 3-sized parts
        let blocks = [
            "###/#.#/###".parse::<Pattern>().unwrap(),
            "#../#../#..".parse::<Pattern>().unwrap(),
            "..#/..#/..#".parse::<Pattern>().unwrap(),
            ".../.../###".parse::<Pattern>().unwrap(),
        ];
        let expected: u32 = blocks.iter().map(|b| b.get_count()).sum();
        let combined = Pattern::recombine(&blocks);
        assert_eq!(combined.rows.len(), 6);
        // No cells altered during recombination
        assert_eq!(combined.get_count(), expected);
        // Verify that the next break-up of the combined pattern breaks it into nine 2-sized blocks.
        let parts = combined.break_up();
        assert_eq!(parts.len(), 9);
        assert_eq!(parts.iter().map(|p| p.get_count()).sum::<u32>(), expected);
    }
}
