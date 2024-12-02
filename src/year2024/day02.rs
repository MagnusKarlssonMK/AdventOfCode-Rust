use std::cmp::Ordering;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    reports: Vec<Vec<usize>>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            reports: input.lines()
                .map(|line|
                    line.split_whitespace()
                    .map(|nbr| nbr.parse().unwrap())
                    .collect())
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.reports.iter()
            .filter(|levels| is_safe(levels))
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.reports.iter()
            .filter(|levels|
                if is_safe(levels) {
                    true
                } else {
                    is_almost_safe(levels)
                } )
            .count()
    }
}

fn is_safe(levels: &[usize]) -> bool {
    let diffs: Vec<(usize, Ordering)> = levels.windows(2).map(|w| (w[1].abs_diff(w[0]), w[1].cmp(&w[0]))).collect();
    diffs.iter().all(|&(v, o)| (1..=3).contains(&v) && o == diffs.first().unwrap().1)
}

fn is_almost_safe(levels: &[usize]) -> bool {
    for n in 0..levels.len() {
        let removed: Vec<usize> = levels.iter().enumerate().filter(|(i, _)| *i != n).map(|(_, level)| *level).collect();
        if is_safe(&removed) {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
