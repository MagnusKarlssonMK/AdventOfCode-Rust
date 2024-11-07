use md5::{Digest, Md5};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData<'a> {
    secret_key: &'a str
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self { secret_key: &input }
    }

    fn find_lowest_number(&self, prefix: &str) -> usize {
        let mut suffix: usize = 0;
        loop {
            let mut candidate = self.secret_key.to_string();
            candidate.push_str(&suffix.to_string());
            if format!("{:x}", Md5::digest(&candidate)).starts_with(&prefix) {
                break;
            }
            suffix += 1;
        }
        suffix
    }

    fn solve_part1(&self) -> usize {
        self.find_lowest_number("00000")
    }

    fn solve_part2(&self) -> usize {
        self.find_lowest_number("000000")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("abcdef");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 609043);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("pqrstuv");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1048970);
    }
}
