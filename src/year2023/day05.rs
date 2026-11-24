//! # 2023 day 5 - If You Give A Seed A Fertilizer
//!
//! Each seed passes through every category map in order. Part 1 maps the listed seeds
//! individually; part 2 interprets adjacent seed pairs as ranges and maps whole ranges at a time.
use crate::aoc_util::error::{AocError, OptionExt};
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

struct MapEntry {
    destination: u64,
    source: u64,
    /// exclusive end of the source interval
    end: u64,
}

impl FromStr for MapEntry {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split_whitespace();
        let destination = fields.next().ctx("map range destination")?.parse()?;
        let source: u64 = fields.next().ctx("map range source")?.parse()?;
        let range: u64 = fields.next().ctx("map range length")?.parse()?;
        if fields.next().is_some() {
            return Err(AocError::Invalid(format!("map range: {s}")));
        }
        let end = source
            .checked_add(range)
            .ok_or_else(|| AocError::Overflow("map range end".to_string()))?;
        Ok(Self {
            destination,
            source,
            end,
        })
    }
}

struct CategoryMap {
    ranges: Vec<MapEntry>,
}

impl FromStr for CategoryMap {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, body) = s.split_once('\n').ctx("map header missing")?;
        if !header.ends_with(" map:") {
            return Err(AocError::Invalid(format!(
                "unexpected map header: {header:?}"
            )));
        }
        let ranges = body.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { ranges })
    }
}

impl CategoryMap {
    fn map_numbers(&self, numbers: &mut [u64]) -> Result<(), AocError> {
        for number in numbers {
            for r in &self.ranges {
                if (r.source..r.end).contains(number) {
                    // Reorder so the intermediate value cannot go below zero, then check the sum.
                    *number = (*number - r.source)
                        .checked_add(r.destination)
                        .ok_or_else(|| AocError::Overflow("mapped value".to_string()))?;
                    break;
                }
            }
        }
        Ok(())
    }

    fn map_nbr_ranges(&self, nbr_ranges: &[(u64, u64)]) -> Result<Vec<(u64, u64)>, AocError> {
        let mut next_nbr_ranges = Vec::new();
        let mut map_buffer = nbr_ranges.to_vec();
        for r in &self.ranges {
            let mut not_mapped = Vec::new();

            for (low, high) in &map_buffer {
                let intersection_low = r.source.max(*low);
                let intersection_high = r.end.min(*high);

                if intersection_low < intersection_high {
                    // At least some overlap
                    let mapped_low = (intersection_low - r.source)
                        .checked_add(r.destination)
                        .ok_or_else(|| AocError::Overflow("mapped range".to_string()))?;
                    let mapped_high = (intersection_high - r.source)
                        .checked_add(r.destination)
                        .ok_or_else(|| AocError::Overflow("mapped range".to_string()))?;
                    next_nbr_ranges.push((mapped_low, mapped_high));
                    if *low < intersection_low {
                        // The current range sticks out below the map
                        not_mapped.push((*low, intersection_low));
                    }
                    if intersection_high < *high {
                        // The current range sticks out above the map
                        not_mapped.push((intersection_high, *high));
                    }
                } else {
                    // No overlap
                    not_mapped.push((*low, *high));
                }
            }
            map_buffer = not_mapped;
        }
        next_nbr_ranges.append(&mut map_buffer);
        Ok(next_nbr_ranges)
    }
}

struct InputData {
    seeds: Vec<u64>,
    maps: Vec<CategoryMap>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let seedline = blocks.next().ctx("seeds missing")?;
        let seedline = seedline
            .strip_prefix("seeds:")
            .ctx("expected 'seeds: <numbers>'")?;
        let seeds: Vec<u64> = seedline
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        if seeds.is_empty() {
            return Err(AocError::Invalid("no seeds".to_string()));
        }
        if !seeds.len().is_multiple_of(2) {
            return Err(AocError::Invalid("seed count must be even".to_string()));
        }
        let maps = blocks.map(|b| b.parse()).collect::<Result<_, _>>()?;
        Ok(Self { seeds, maps })
    }
}

impl InputData {
    fn solve_part1(&self) -> Result<u64, AocError> {
        let mut seed_values = self.seeds.clone();

        for category in &self.maps {
            category.map_numbers(&mut seed_values)?;
        }
        Ok(*seed_values.iter().min().ok_or(AocError::NoSolution)?)
    }

    fn solve_part2(&self) -> Result<u64, AocError> {
        let mut seed_ranges = Vec::with_capacity(self.seeds.len() / 2);
        for pair in self.seeds.as_chunks::<2>().0 {
            let end = pair[0]
                .checked_add(pair[1])
                .ok_or_else(|| AocError::Overflow("seed range end".to_string()))?;
            seed_ranges.push((pair[0], end));
        }
        for category in &self.maps {
            seed_ranges = category.map_nbr_ranges(&seed_ranges)?;
        }
        Ok(*seed_ranges
            .iter()
            .map(|(low, _)| low)
            .min()
            .ok_or(AocError::NoSolution)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    /// A seed range that sticks out on both sides of the first mapping, with a second mapping that
    /// lifts the leading fragment above the trailing one, so part 1 (the single seeds 0 and 12)
    /// and part 2 (the range 0-11) disagree.
    const TEST_CUSTOM_1: &str = "seeds: 0 12

custom map:
102 2 8
50 0 1";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 35);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 12);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 46);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 1);
    }

    #[test]
    fn parser_rejects_invalid() {
        for input in [
            "seeds:",
            "seeds: 1",
            "foo: 1 2",
            "seeds: 1 2\n\nnot a map\n1 2 3",
            "seeds: 1 2\n\ncustom map:\n1 2",
        ] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
    }

    #[test]
    fn overflow_is_rejected() {
        // seed start + count overflows u64
        let seed_end = "seeds: 18446744073709551615 2\n\ncustom map:\n0 0 1";
        assert!(matches!(
            seed_end.parse::<InputData>().and_then(|d| d.solve_part2()),
            Err(AocError::Overflow(_))
        ));

        // source + range overflows at parse time
        let map_end = "seeds: 1 2\n\ncustom map:\n0 18446744073709551615 2";
        assert!(matches!(
            map_end.parse::<InputData>(),
            Err(AocError::Overflow(_))
        ));

        // mapped value overflows even though the offset is representable
        let mapped = "seeds: 5 1\n\ncustom map:\n18446744073709551615 0 10";
        assert!(matches!(
            mapped.parse::<InputData>().and_then(|d| d.solve_part1()),
            Err(AocError::Overflow(_))
        ));
    }
}
