//! # 2015 day 10 - Elves Look, Elves Say
//!
//! Straight transfer from my python solution => brute force
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    start_numbers: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { start_numbers: s })
    }
}

impl<'a> InputData<'a> {
    fn get_generated_length(&self, rounds: usize) -> usize {
        let mut sequence = self.start_numbers.to_string();
        for _ in 0..rounds {
            let mut new_seq = String::new();
            let mut count = 0;
            let mut currentchar = ' ';
            for c in sequence.chars() {
                if c == currentchar {
                    count += 1;
                } else {
                    if count > 0 {
                        new_seq += &(count.to_string() + &currentchar.to_string());
                    }
                    count = 1;
                    currentchar = c;
                }
            }
            if count > 0 {
                new_seq += &(count.to_string() + &currentchar.to_string());
            }
            sequence = new_seq;
        }
        sequence.len()
    }

    fn solve_part1(&self) -> usize {
        self.get_generated_length(40)
    }

    fn solve_part2(&self) -> usize {
        self.get_generated_length(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "1";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.get_generated_length(1), 2);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "11";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.get_generated_length(1), 2);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "21";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.get_generated_length(1), 4);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "1211";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.get_generated_length(1), 6);
    }

    #[test]
    fn part1_example_5() {
        let testdata = "111221";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.get_generated_length(1), 6);
    }
}
