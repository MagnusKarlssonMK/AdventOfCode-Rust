pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    santa_strings: Vec<String>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { santa_strings: input.lines().map(String::from).collect() }
    }

    fn solve_part1(&self) -> usize {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        fn is_nice(word: &str) -> bool {
            let mut previous_char: Option<char> = None;
            let mut vowel_counter = 0;
            let mut double_char = false;
            for c in word.chars() {
                if VOWELS.contains(&c) {
                    vowel_counter += 1;
                }
                if let Some(prev) = previous_char {
                    if (prev == 'a' && c == 'b') || (prev == 'c' && c == 'd') ||
                           (prev == 'p' && c == 'q') || (prev == 'x' && c == 'y') {
                            return false;
                    }
                    if c == prev {
                        double_char = true;
                    }
                }
                previous_char = Some(c);
            }
            vowel_counter >= 3 && double_char
        }
        self.santa_strings.iter().filter(|word| is_nice(word)).count()
    }

    fn solve_part2(&self) -> usize {
        fn is_nice(word: &str) -> bool {
            if word.chars().zip(word.chars().skip(2)).any(|(c1, c2)| c1 == c2) {
                for idx in 0..word.len() - 1 {
                    let candidate = &word[idx..idx+2];
                    if word[..idx].contains(&candidate) || word[idx+2..].contains(&candidate) {
                        return true;
                    }
                }
            }
            false
        }
        self.santa_strings.iter().filter(|word| is_nice(word)).count()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("ugknbfddgicrmopn");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("aaa");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("jchzalrnumimnmhp");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("haegwjzuvuyypxyu");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part1_example_5() {
        let testdata = String::from("dvszwmarrgswjxmb");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 0);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("qjhvhtzxzqqjkmpb");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("xxyxx");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from("uurcxstgmygtbstg");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part2_example_4() {
        let testdata = String::from("ieodomkazucvgmuy");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 0);
    }
}
