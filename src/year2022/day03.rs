use std::collections::HashSet;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    rucksacks: Vec<Vec<usize>>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { rucksacks: input.lines()
                .map(|line| line.chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            27 + (c as usize - 'A' as usize)
                        } else {
                            1 + (c as usize - 'a' as usize)
                        }
                    })
                    .collect())
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.rucksacks.iter()
            .map(|rucksack| {
                let (left, right) = rucksack.split_at(rucksack.len() / 2);
                let a: HashSet<usize> = HashSet::from_iter(left.to_vec());
                let b: HashSet<usize> = HashSet::from_iter(right.to_vec());
                let common = a.intersection(&b).next().unwrap();
                *common
            }).sum()
    }

    fn solve_part2(&self) -> usize {
        self.rucksacks
            .chunks(3)
            .map(|group| {
                // Can't figure out how to instersect more than two sets - brute force looping for now...
                let mut badge: usize = 0;
                for a in group[0].clone() {
                    if group[1].contains(&a) && group[2].contains(&a) {
                        badge = a;
                        break;
                    }
                }
                badge
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 157);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 70);
    }
}
