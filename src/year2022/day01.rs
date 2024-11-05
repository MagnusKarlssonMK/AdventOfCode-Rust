use std::usize;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    elfgroups: Vec<usize>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut groups: Vec<usize> = input.split("\n\n")
            .map(|group| group.lines()
                .map(|elf| elf.parse::<usize>().unwrap())
                .sum())
            .collect();
        groups.sort_unstable();
        Self { elfgroups: groups }
    }

    fn solve_part1(&self) -> usize {
        *self.elfgroups.last().unwrap()
    }

    fn solve_part2(&self) -> usize {
        self.elfgroups.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 24000);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 45000);
    }
}
