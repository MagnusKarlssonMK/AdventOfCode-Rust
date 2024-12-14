pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    assignments: Vec<Vec<usize>>,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            assignments: input
                .lines()
                .map(|line| {
                    line.split([',', '-'])
                        .map(|nbr| nbr.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn solve_part1(&self) -> usize {
        self.assignments
            .iter()
            .map(|a| {
                if (a[0] <= a[2] && a[2] <= a[3] && a[3] <= a[1])
                    || (a[2] <= a[0] && a[0] <= a[1] && a[1] <= a[3])
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.assignments
            .iter()
            .map(|a| if a[0] <= a[3] && a[2] <= a[1] { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 4);
    }
}
