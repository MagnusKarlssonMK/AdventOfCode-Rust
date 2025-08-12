//! # 2023 day 5 - If You Give A Seed A Fertilizer
//!
//! No need to actually create any hashmap or anything between different text identifiers.
//! Just dump all the maps in a vector in the parsed order.
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct MapRange {
    destination: usize,
    source: usize,
    range: usize,
}

impl FromStr for MapRange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split_whitespace();
        let destination = numbers.next().unwrap().parse().unwrap();
        let source = numbers.next().unwrap().parse().unwrap();
        let range = numbers.next().unwrap().parse().unwrap();
        Ok(Self {
            destination,
            source,
            range,
        })
    }
}

struct CategoryMap {
    ranges: Vec<MapRange>,
}

impl FromStr for CategoryMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .skip(1)
            .map(|line| MapRange::from_str(line).unwrap())
            .collect();
        Ok(Self { ranges })
    }
}

impl CategoryMap {
    fn map_numbers(&self, numbers: &mut [usize]) {
        for number in numbers.iter_mut() {
            for r in &self.ranges {
                if (r.source..r.source + r.range).contains(number) {
                    *number = *number + r.destination - r.source;
                    // Note: using += doesnt work, since the offset between destination and source can be negative
                    // which causes an unsigned int underflow
                    break;
                }
            }
        }
    }

    fn map_nbr_ranges(&self, nbr_ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut next_nbr_ranges = Vec::new();
        let mut map_buffer = nbr_ranges.to_vec();
        for r in &self.ranges {
            let mut not_mapped = Vec::new();

            for (low, high) in &map_buffer {
                let intersection_low = r.source.max(*low);
                let intersection_high = (r.source + r.range).min(*high);

                if intersection_low < intersection_high {
                    // At least some overlap
                    next_nbr_ranges.push((
                        intersection_low - r.source + r.destination,
                        intersection_high - r.source + r.destination,
                    ));
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
        next_nbr_ranges
    }
}

struct InputData {
    seeds: Vec<usize>,
    maps: Vec<CategoryMap>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let seedline = blocks.next().unwrap();
        let seeds: Vec<usize> = seedline
            .split_whitespace()
            .skip(1)
            .map(|i| i.parse().unwrap())
            .collect();
        let maps = blocks.map(|b| CategoryMap::from_str(b).unwrap()).collect();
        Ok(Self { seeds, maps })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let mut seed_values = self.seeds.clone();

        for category in &self.maps {
            category.map_numbers(&mut seed_values);
        }
        *seed_values.iter().min().unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut seed_ranges: Vec<(usize, usize)> =
            self.seeds.chunks(2).map(|c| (c[0], c[0] + c[1])).collect();

        for category in &self.maps {
            seed_ranges = category.map_nbr_ranges(&seed_ranges);
        }
        *seed_ranges.iter().map(|(low, _)| low).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "seeds: 79 14 55 13

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

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 35);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 46);
    }
}
