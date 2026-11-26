//! # 2021 day 08 - Seven Segment Search
//!
//! Rather than solving this based on the length of the patterns, this uses a frequency-based
//! fingerprint for each pattern. By counting how many times each segment occurs over all defined
//! digits, each segment gets a weight, and we can then create a fingerprint for each digit by
//! taking the sum of all those weights for the segment a digit is made up of. With the given digit
//! layout, these hashes will be unique for each digit. So we can then for each row in the input,
//! use the lefthand side to create the labels (counts), and then use that to create the hashes for
//! the righthand side and match with the original hashes to extract the corresponding digit.
//!
//! Note that if we were to create a custom segment layout with for example just the 2 and the 5,
//! both hashes would be identical. Ending up with identical hashes simply means that the problem
//! is not solveable for that setup; there is not enough information to be able to identify
//! scrambled digits.
use crate::aoc_util::error::AocError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

/// Total number of display segments
const NBR_SEGMENTS: u8 = 7;
/// The segment layout for each digit, 0..=9 in order
const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];
const P1_DIGITS: [usize; 4] = [1, 4, 7, 8];

/// Turns a digit like "acdeg" into a bitmask, 'a' in LSB
fn digit_mask(s: &str) -> Result<u8, AocError> {
    let mut result: u8 = 0;
    for b in s.bytes() {
        if !(b'a'..b'a' + NBR_SEGMENTS).contains(&b) {
            return Err(AocError::Invalid(format!("invalid segments in: {s}")));
        }
        result += 1 << (b - b'a');
    }
    if result.count_ones() != s.len() as u32 {
        Err(AocError::Invalid(format!("duplicated segments in: {s}")))
    } else {
        Ok(result)
    }
}

/// Takes a slice of masks and converts it into segment occurrence counts
fn counts(masks: &[u8]) -> [u8; NBR_SEGMENTS as usize] {
    let mut counts = [0u8; NBR_SEGMENTS as usize];
    for (i, bit) in counts.iter_mut().enumerate() {
        for m in masks {
            *bit += (m >> i) & 1;
        }
    }
    counts
}

fn signatures<const N: usize>(masks: &[u8; N], counts: &[u8; NBR_SEGMENTS as usize]) -> [u8; N] {
    masks
        .iter()
        .map(|m| {
            counts
                .iter()
                .enumerate()
                .filter(|&(i, _)| (m >> i) & 1 > 0)
                .map(|(_, c)| *c)
                .sum()
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("fixed-size input")
}

struct DisplayNotes {
    /// The lefthand side of `|`; the patterns of the scrambled digits
    scrambled_patterns: [u8; DIGITS.len()],
    /// The righthand side of `|`; the scrambled digits displayed
    displayed_digits: [u8; 4],
}

impl FromStr for DisplayNotes {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once('|')
            .ok_or_else(|| AocError::Invalid(format!("invalid note: {s}")))?;
        Ok(Self {
            scrambled_patterns: left
                .split_whitespace()
                .map(digit_mask)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|_| {
                    AocError::Invalid(format!("expected {} patterns: {s}", DIGITS.len()))
                })?,
            displayed_digits: right
                .split_whitespace()
                .map(digit_mask)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|_| AocError::Invalid(format!("expected four digits: {s}")))?,
        })
    }
}

struct InputData {
    /// The hash signatures for each unscrambled digit, 0..=9 in order
    signatures: [u8; DIGITS.len()],
    notes: Vec<DisplayNotes>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digit_sigs: [u8; DIGITS.len()] = DIGITS
            .iter()
            .map(|d| digit_mask(d))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .expect("standard digit signatures");
        let c = counts(&digit_sigs);
        let signatures = signatures(&digit_sigs, &c);
        Ok(Self {
            signatures,
            notes: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    /// Returns the descrambled digit (index == digit)
    fn descramble(&self, signatures: &[u8]) -> Result<Vec<usize>, AocError> {
        signatures
            .iter()
            .map(|s| {
                self.signatures
                    .iter()
                    .position(|i| *i == *s)
                    .ok_or(AocError::NoSolution)
            })
            .collect::<Result<_, _>>()
    }

    fn solve(&self) -> Result<(usize, usize), AocError> {
        let mut p1 = 0;
        let mut p2 = 0;
        for note in &self.notes {
            let scrambled_counts = counts(&note.scrambled_patterns);
            let scrambled_signatures = signatures(&note.displayed_digits, &scrambled_counts);
            let descrambled_digits = self.descramble(&scrambled_signatures)?;
            p1 += descrambled_digits
                .iter()
                .filter(|&x| P1_DIGITS.contains(x))
                .count();
            p2 += descrambled_digits
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit);
        }
        Ok((p1, p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const TEST_OFFICIAL_2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn parts1_2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 0);
        assert_eq!(p2, 5353);
    }

    #[test]
    fn parts1_2_official_2() {
        let solution_data: InputData = TEST_OFFICIAL_2.parse().unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 26);
        assert_eq!(p2, 61229);
    }

    #[test]
    fn parser_invalid_input() {
        for input in [
            "fabcd edb fdgacbe cefdb",                  // missing delimiter
            "fabcd edb | fdghcbe cefdb",                // invalid character, upper bound
            "faXcd edb | fdgacbe cefdb",                // invalid character, lower bound
            "fabcd edb | fdgacfe cefdb",                // duplicated character
            "ab ab ab ab ab ab ab ab ab | ab ab ab ab", // too few patterns
            "ab ab ab ab ab ab ab ab ab ab | ab ab ab", // too few digits
        ] {
            assert!(
                input.parse::<InputData>().is_err(),
                "expected error for {input:?}"
            );
        }
    }

    #[test]
    fn solvers_no_solution() {
        let solution_data: InputData = "ab ab ab ab ab ab ab ab ab ab | ab ab ab ab"
            .parse()
            .unwrap();
        assert_eq!(solution_data.solve().unwrap_err(), AocError::NoSolution);
    }
}
