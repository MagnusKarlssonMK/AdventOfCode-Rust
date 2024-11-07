pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    fish_states: [usize; 9],
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let mut states = [0_usize; 9];
        input.split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .for_each(|n| states[n] += 1);
        Self {
            fish_states: states
        }
    }

    fn simulate_fishies(&self, days: usize) -> usize {
        let mut states = self.fish_states;
        for d in 0..days {
            states[(d + 7) % 9] += states[d % 9]
        }
        states.iter().sum()
    }

    fn solve_part1(&self) -> usize {
        self.simulate_fishies(80)
    }

    fn solve_part2(&self) -> usize {
        self.simulate_fishies(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("3,4,3,1,2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 5934);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("3,4,3,1,2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 26984457539);
    }
}
