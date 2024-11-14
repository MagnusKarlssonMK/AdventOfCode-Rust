pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
struct Password {
    nbr_range: (usize, usize),
    letter: char,
    password: String
}

impl Password {
    fn parse_str(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let (r1, r2) = words.next().unwrap().split_once('-').unwrap();
        Self {
            nbr_range: (r1.parse().unwrap(), r2.parse().unwrap()),
            letter: words.next().unwrap().trim_matches(':').parse().unwrap(),
            password: words.next().unwrap().parse().unwrap() }
    }

    fn is_valid_old_policy(&self) -> bool {
        (self.nbr_range.0..=self.nbr_range.1)
            .contains(&self.password
                .chars()
                .filter(|c| *c == self.letter)
                .count())
    }

    fn is_valid_new_policy(&self) -> bool {
        if self.password.len() < self.nbr_range.0 {
            false
        } else {
            let first = self.letter == self.password.chars()
                .nth(self.nbr_range.0 - 1).unwrap();
            let second = self.password.len() >= self.nbr_range.1 &&
                self.letter == self.password.chars().nth(self.nbr_range.1 - 1).unwrap();
            first ^ second
        }
    }
}

struct InputData {
    passwords: Vec<Password>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            passwords: input
                .lines()
                .map(|line| Password::parse_str(line))
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.passwords
            .iter()
            .filter(|p| p.is_valid_old_policy())
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.passwords
            .iter()
            .filter(|p| p.is_valid_new_policy())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }
}
