//! # 2020 day 7 - Handy Haversacks
//!
//! Recursive solution. In theory, these recursive functions would benefit from a memo since the
//! same bags will be recursed multiple times. But the input data set is so small that it gives no
//! benefit in performance, if anything it even seems to be a tiny bit slower. Also, for part 1 it
//! might be cleaner to work on a reverse hashmap, but the current solution is simple enough and
//! gets the job done.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1()?.to_string(),
        solution_data.solve_part2()?.to_string(),
    ))
}

const SHINY_GOLD: &str = "shiny gold";

struct InputData<'a> {
    bags: HashMap<&'a str, Vec<(&'a str, usize)>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        fn parse_color(s: &str) -> Result<&str, AocError> {
            let s = s.trim_end_matches('.');
            s.strip_suffix(" bags")
                .or_else(|| s.strip_suffix(" bag"))
                .ctx("bag suffix")
        }

        fn parse_content(s: &str) -> Result<(&str, usize), AocError> {
            let (nbr, color) = s.split_once(' ').ctx("bag count separator")?;
            Ok((parse_color(color)?, nbr.parse()?))
        }

        let mut bags = HashMap::new();
        for line in s.lines() {
            let (left, right) = line.split_once(" bags contain ").ctx("bags contain")?;
            let contents = if right == "no other bags." {
                Vec::new()
            } else {
                right
                    .split(", ")
                    .map(parse_content)
                    .collect::<Result<_, _>>()?
            };
            bags.insert(left, contents);
        }
        Ok(Self { bags })
    }
}

impl InputData<'_> {
    fn contents_of(&self, bag: &str) -> Result<&[(&str, usize)], AocError> {
        self.bags
            .get(bag)
            .map(Vec::as_slice)
            .ok_or(AocError::NoSolution)
    }

    fn bag_contains_color(&self, bag: &str, target: &str) -> Result<bool, AocError> {
        for &(content, _) in self.contents_of(bag)? {
            if content == target || self.bag_contains_color(content, target)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn count_bag(&self, bag: &str) -> Result<usize, AocError> {
        let mut total = 0;
        for &(content, count) in self.contents_of(bag)? {
            total += count * (1 + self.count_bag(content)?);
        }
        Ok(total)
    }

    fn solve_part1(&self) -> Result<usize, AocError> {
        let mut count = 0;
        for bag in self.bags.keys() {
            if self.bag_contains_color(bag, SHINY_GOLD)? {
                count += 1;
            }
        }
        Ok(count)
    }

    fn solve_part2(&self) -> Result<usize, AocError> {
        self.count_bag(SHINY_GOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_OFFICIAL_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1().unwrap(), 4);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 32);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2().unwrap(), 126);
    }
}
