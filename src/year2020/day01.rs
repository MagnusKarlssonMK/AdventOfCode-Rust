pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

const TARGET: usize = 2020;

struct InputData {
    reports: Vec<usize>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { reports:
            input.lines()
                .map(|line| line.parse().unwrap())
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        let nbr_reports = self.reports.len();
        for i in 0..nbr_reports-1 {
            for j in i+1..nbr_reports {
                if self.reports[i] + self.reports[j] == TARGET {
                    return self.reports[i] * self.reports[j];
                }
            }
        }
        0
    }

    fn solve_part2(&self) -> usize {
        let nbr_reports = self.reports.len();
        for i in 0..nbr_reports-2 {
            for j in i+1..nbr_reports-1 {
                let pairsum = self.reports[i] + self.reports[j];
                if pairsum < TARGET {
                    for k in j+1..nbr_reports {
                        if pairsum + self.reports[k] == TARGET {
                            return self.reports[i] * self.reports[j] * self.reports[k];
                        }
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("1721\n979\n366\n299\n675\n1456");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 514579);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("1721\n979\n366\n299\n675\n1456");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 241861950);
    }
}
