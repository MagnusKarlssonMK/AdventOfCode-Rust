pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData<'a> {
    program: &'a str
}

impl <'a>InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self {
            program: input
        }
    }

    fn solve_part1(&self) -> usize {
        calculate(self.program)
    }

    fn solve_part2(&self) -> usize {
        let parts: Vec<&str> = self.program.split("don't()").collect();
        let mut do_parts: Vec<String> = Vec::new();
        do_parts.push(parts.first().unwrap().to_string());
        for p in parts.iter().skip(1) {
            if let Some((_, d)) = p.split_once("do()") {
                do_parts.push(d.to_string());
            }
        }
        do_parts.iter().map(|d| calculate(d)).sum()
    }
}

fn calculate(input: &str) -> usize {
    input.split("mul(")
        .map(|part|
            if let Some((p, _)) = part.split_once(')') {
                if let Some((n1, n2)) = p.split_once(',') {
                    let nbr1: usize = n1.parse().unwrap_or_default();
                    let nbr2: usize = n2.parse().unwrap_or_default();
                    nbr1 * nbr2
                } else {
                    0
                }
            } else {
                0
            })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 161);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 48);
    }
}
