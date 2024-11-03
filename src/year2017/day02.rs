pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    spreadheet: Vec<Vec<usize>>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { spreadheet: input
            .lines()
            .map(|row| {
                let mut values: Vec<_> = row
                    .split_whitespace()
                    .map(|nbr|
                        nbr
                        .parse()
                        .unwrap())
                    .collect();
                values.sort_unstable();
                values })
            .collect() }
    }

    fn solve_part1(&self) -> usize {
        self.spreadheet.iter()
            .map(|row| row.last().unwrap() - row.first().unwrap())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.spreadheet.iter()
            .map(|row| {
                let mut result: usize = 0;
                for i in 0..row.len() - 1 {
                    for j in i+1..row.len() {
                        if row[j] % row[i] == 0 {
                            result += row[j] / row[i];
                            break;
                        }
                    }
                }
                result
            })
            .sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("5 1 9 5\n7 5 3\n2 4 6 8");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("5 9 2 8\n9 4 7 3\n3 8 6 5");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 9);
    }
}
