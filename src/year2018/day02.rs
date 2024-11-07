use std:: collections::HashMap;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData<'a> {
    ids: Vec<&'a str>
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self {
            ids: input.lines().collect()
        }
    }

    fn solve_part1(&self) -> usize {
        let mut twos: usize = 0;
        let mut threes: usize = 0;
        for id in &self.ids {
            let mut counts: HashMap<char, usize> = HashMap::new();
            let mut two: isize = 0;
            let mut three: isize = 0;
            for c in id.chars() {
                if let Some(x) = counts.get_mut(&c) {
                    match *x {
                        1 => two += 1,
                        2 => {two -= 1; three += 1},
                        3 => three -= 1,
                        _ => ()
                    }
                    *x += 1;
                } else {
                    counts.insert(c, 1);
                }
            }
            if two > 0 {
                twos += 1;
            }
            if three > 0 {
                threes += 1;
            }
        }
        twos * threes
    }

    fn solve_part2(&self) -> String {
        for left in 0..self.ids.len()-1 {
            for right in 1..self.ids.len() {
                let common: String = self.ids[left]
                    .chars()
                    .zip(self.ids[right].chars())
                    .filter_map(|(lc, rc)| if lc == rc { Some(lc)} else { None })
                    .collect();
                if common.len() == self.ids[left].len() - 1 {
                    return common;
                }
            }
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 12);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), "fgij");
    }
}
