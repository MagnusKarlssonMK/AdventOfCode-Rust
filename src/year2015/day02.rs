pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    gifts: Vec<(usize, usize, usize)>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        // Parse and store the dimensions sorted low to high - the order in the input
        // doesn't matter, so save some effort later by sorting it already here.
        Self { gifts: input.lines()
            .map(|line| {
                let mut sides = line.split('x')
                                                .map(|digit| digit.parse::<usize>().unwrap())
                                                .collect::<Vec<_>>();
                sides.sort_unstable();
                if sides.len() != 3 { (0, 0, 0) } else { (sides[0], sides[1], sides[2]) }
            })
            .collect() }
    }

    fn solve_part1(&self) -> usize {
        self.gifts
            .iter()
            .map(|(l, w, h)|
                3 * (l * w) + 2 * h * (w + l)
            )
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.gifts
            .iter()
            .map(|(l, w, h)|
                2 * (l + w) + (l * w * h)
            )
            .sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("2x3x4");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 58);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("1x1x10");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 43);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("2x3x4");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 34);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("1x1x10");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 14);
    }
}
