//! # 2023 day 12 - Hot Springs
//!
//! Bottom-up dynamic programming over {position in springs, index of next group}. The table is a
//! flat `Vec` indexed by those integers, so there is no need for string copying or hashing. The
//! rows in the input are independent and can be processed independently over multiple threads.
use crate::aoc_util::{error::AocError, thread::spawn_jobs};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct SpringRow {
    springs: Vec<u8>,
    group_sizes: Vec<usize>,
}

impl FromStr for SpringRow {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, group_sizes) = s
            .split_once(' ')
            .ok_or_else(|| AocError::Invalid(s.to_string()))?;
        if !springs.bytes().all(|b| matches!(b, b'.' | b'#' | b'?')) {
            return Err(AocError::Invalid(s.to_string()));
        }
        Ok(Self {
            springs: springs.as_bytes().to_vec(),
            group_sizes: group_sizes
                .split(',')
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl SpringRow {
    /// Number of valid arrangements when the row is repeated `copies` times, joined by `?`.
    fn calculate_combinations(&self, copies: usize) -> usize {
        // Unfold the row. A trailing '.' means a group can always look at the byte following it,
        // without special-casing the end of the string.
        let mut springs = Vec::with_capacity((self.springs.len() + 1) * copies + 1);
        let mut groups = Vec::with_capacity(self.group_sizes.len() * copies);
        for c in 0..copies {
            if c > 0 {
                springs.push(b'?');
            }
            springs.extend_from_slice(&self.springs);
            groups.extend_from_slice(&self.group_sizes);
        }
        springs.push(b'.');

        let n = springs.len();
        let g = groups.len();
        let width = g + 1;

        // dots[i] = number of '.' in springs[..i], to test "no '.' in range"
        let mut dots = vec![0usize; n + 1];
        for (i, &b) in springs.iter().enumerate() {
            dots[i + 1] = dots[i] + usize::from(b == b'.');
        }

        // dp[i * width + j] = ways to arrange springs[i..] using groups[j..]
        let mut dp = vec![0usize; (n + 1) * width];
        dp[n * width + g] = 1;
        for i in (0..n).rev() {
            for j in 0..=g {
                let mut total = 0;
                if springs[i] != b'#' {
                    total += dp[(i + 1) * width + j];
                }
                // A group starts at springs[i]
                if j < g {
                    let end = i + groups[j];
                    if end < n && dots[end] == dots[i] && springs[end] != b'#' {
                        total += dp[(end + 1) * width + j + 1];
                    }
                }
                dp[i * width + j] = total;
            }
        }
        dp[0]
    }
}

struct InputData {
    rows: Vec<SpringRow>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn sum_combinations(&self, copies: usize) -> usize {
        spawn_jobs(self.rows.iter().collect(), |rows: Vec<&SpringRow>| {
            rows.iter()
                .map(|row| row.calculate_combinations(copies))
                .sum::<usize>()
        })
        .into_iter()
        .sum()
    }

    fn solve_part1(&self) -> usize {
        self.sum_combinations(1)
    }

    fn solve_part2(&self) -> usize {
        self.sum_combinations(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "???.### 1,1,3";
    const TEST_OFFICIAL_2: &str = ".??..??...?##. 1,1,3";
    const TEST_OFFICIAL_3: &str = "?#?#?#?#?#?#?#? 1,3,1,6";
    const TEST_OFFICIAL_4: &str = "????.#...#... 4,1,1";
    const TEST_OFFICIAL_5: &str = "????.######..#####. 1,6,5";
    const TEST_OFFICIAL_6: &str = "?###???????? 3,2,1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_official_4() {
        let solution_data: InputData = TEST_OFFICIAL_4.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_official_5() {
        let solution_data: InputData = TEST_OFFICIAL_5.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_official_6() {
        let solution_data: InputData = TEST_OFFICIAL_6.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 10);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 16384);
    }

    #[test]
    fn part2_official_3() {
        let solution_data: InputData = TEST_OFFICIAL_3.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_official_4() {
        let solution_data: InputData = TEST_OFFICIAL_4.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 16);
    }

    #[test]
    fn part2_official_5() {
        let solution_data: InputData = TEST_OFFICIAL_5.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 2500);
    }

    #[test]
    fn part2_official_6() {
        let solution_data: InputData = TEST_OFFICIAL_6.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 506250);
    }
}
