//! # 2020 day 7 - Handy Haversacks
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    bags: HashMap<&'a str, Vec<(&'a str, usize)>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut bags = HashMap::new();
        for line in s.lines() {
            let (left, right) = line.split_once(" bags contain ").unwrap();
            bags.insert(
                left,
                right
                    .split(", ")
                    .filter_map(|item| {
                        let (nbr, bagstr) = item.split_once(' ').unwrap();
                        if let Ok(c) = nbr.parse() {
                            let color = bagstr
                                .trim_end_matches('.')
                                .trim_end_matches('s')
                                .trim_end_matches(" bag");
                            Some((color, c))
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }
        Ok(Self { bags })
    }
}

impl InputData<'_> {
    fn bag_contains_color(&self, bag: &str, target: &str) -> bool {
        for &(content, _) in self.bags.get(bag).unwrap().iter() {
            if content == target || self.bag_contains_color(content, target) {
                return true;
            }
        }
        false
    }

    fn count_bag(&self, bag: &str) -> usize {
        self.bags
            .get(bag)
            .unwrap()
            .iter()
            .map(|&(b, c)| c * (1 + self.count_bag(b)))
            .sum()
    }

    fn solve_part1(&self) -> usize {
        self.bags
            .keys()
            .filter(|&&bag| self.bag_contains_color(bag, "shiny gold"))
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.count_bag("shiny gold")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_DATA_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::try_from(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 32);
    }

    #[test]
    fn part2_example_2() {
        let solution_data = InputData::try_from(TEST_DATA_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 126);
    }
}
