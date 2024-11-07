use std::{collections::HashSet, usize};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    width: usize,
    numbers: Vec<usize>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let nbrs: Vec<&str> = input.lines().collect();
        Self {
            width: nbrs[0].len(),
            numbers: nbrs.iter().map(|n| usize::from_str_radix(&n, 2).unwrap()).collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        let mut gamma: usize = 0;
        let mut epsilon: usize = 0;
        for bit in 0..self.width {
            let mask: usize = 1 << bit;
            let onecount: usize = self.numbers.iter().map(|nbr| if nbr & mask > 0 {1} else {0} ).sum();
            if onecount > self.numbers.len() / 2 {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }
        }
        gamma * epsilon
    }

    fn solve_part2(&self) -> usize {
        let mut oxygen_candidates: HashSet<usize> = HashSet::new();
        let mut scrubber_candidates: HashSet<usize> = HashSet::new();
        for i in 0..self.numbers.len() {
            oxygen_candidates.insert(i);
            scrubber_candidates.insert(i);
        }
        // This can probably be made a lot shorter with hashmap instead of two hashsets, and also some way to re-use almost duplicated code.
        for bit in (0..self.width).rev() {
            let mask: usize = 1 << bit;
            if oxygen_candidates.len() > 1 {
                let mut ones: HashSet<usize> = HashSet::new();
                let mut zeroes: HashSet<usize> = HashSet::new();
                for c in oxygen_candidates.iter() {
                    if self.numbers[*c] & mask > 0 {
                        ones.insert(*c);
                    } else {
                        zeroes.insert(*c);
                    }
                }
                if ones.len() >= zeroes.len() {
                    for z in zeroes {
                        oxygen_candidates.remove(&z);
                    }
                } else {
                    for o in ones {
                        oxygen_candidates.remove(&o);
                    }
                }
            }
            if scrubber_candidates.len() > 1 {
                let mut ones: HashSet<usize> = HashSet::new();
                let mut zeroes: HashSet<usize> = HashSet::new();
                for c in scrubber_candidates.iter() {
                    if self.numbers[*c] & mask > 0 {
                        ones.insert(*c);
                    } else {
                        zeroes.insert(*c);
                    }
                }
                if zeroes.len() > ones.len() {
                    for z in zeroes {
                        scrubber_candidates.remove(&z);
                    }
                } else {
                    for o in ones {
                        scrubber_candidates.remove(&o);
                    }
                }
            }
        }
        self.numbers[*oxygen_candidates.iter().next().unwrap()] * self.numbers[*scrubber_candidates.iter().next().unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 198);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 230);
    }
}
