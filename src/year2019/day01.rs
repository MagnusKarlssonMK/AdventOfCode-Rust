use std::cmp::max;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

fn calc_mass(mass: &usize, countfuel: bool) -> usize {
    let fuel_own_mass = max(0, (*mass as i32 / 3) - 2) as usize;
    if fuel_own_mass == 0 || !countfuel {
        fuel_own_mass
    } else {
        fuel_own_mass + calc_mass(&fuel_own_mass, countfuel)
    }
}

struct InputData {
    data: Vec<usize>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self { data:
            input.lines()
                .map(|line| line.parse().unwrap())
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.data.iter().map(|m| calc_mass(m, false)).sum()
    }

    fn solve_part2(&self) -> usize {
        self.data.iter().map(|m| calc_mass(m, true)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("12");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("14");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 2);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("1969");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 654);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("100756");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 33583);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("14");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from("1969");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 966);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from("100756");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 50346);
    }
}
