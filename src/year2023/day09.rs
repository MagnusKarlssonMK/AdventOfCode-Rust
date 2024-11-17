pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    numbers: Vec<Vec<isize>>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            numbers: input.lines()
                .map(|line| line.split_whitespace()
                    .map(|nbr| nbr.parse().unwrap())
                    .collect())
                .collect()
        }
    }

    fn solve_part1(&self) -> isize {
        self.numbers.iter()
            .map(|line| find_next_number(line))
            .sum()
    }

    fn solve_part2(&self) -> isize {
        self.numbers.iter()
            .map(|line| {
                let r: Vec<isize> = line.iter().rev().copied().collect();
                find_next_number(&r)})
            .sum()
    }
}

fn find_next_number(nbrs: &[isize]) -> isize {
    if nbrs.iter().filter(|n| **n != 0).count() == 0 {
        0
    } else {
        let mut nextlevellist = Vec::new();
        for i in 1..nbrs.len() {
            nextlevellist.push(nbrs[i] - nbrs[i-1]);
        }
        nbrs.last().unwrap() + find_next_number(&nextlevellist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 114);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
