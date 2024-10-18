// Parking this for now, can't get md5 crate imports to work, looks like md5 has vanished from crypto crates...

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    secret_key: String
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { secret_key: input.to_string() }
    }

    fn solve_part1(&self) -> u32 {
        println!("{}", self.secret_key);
        //let testdata = "abcdef609043";
        1
    }

    fn solve_part2(&self) -> u32 {
        1
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
