use std::cmp::Ordering;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    left: Vec<usize>,
    right: Vec<usize>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        for line in input.lines() {
            let mut nbrs = line.split_whitespace();
            left.push(nbrs.next().unwrap().parse().unwrap());
            right.push(nbrs.next().unwrap().parse().unwrap());
        }
        left.sort_unstable();
        right.sort_unstable();
        Self { left, right }
    }

    fn solve_part1(&self) -> usize {
        self.left.iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut right_idx = 0;
        self.left.iter().map(|left_nbr| {
            let mut score = 0;
            for right_nbr in self.right.iter().skip(right_idx) {
                match right_nbr.cmp(left_nbr) {
                    Ordering::Greater => break,
                    Ordering::Equal => score += left_nbr,
                    Ordering::Less => right_idx += 1,
                }
            }
            score
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
"3   4
4   3
2   5
1   3
3   9
3   3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 11);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
"3   4
4   3
2   5
1   3
3   9
3   3");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 31);
    }
}
