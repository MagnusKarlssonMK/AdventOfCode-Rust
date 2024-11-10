pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

fn get_median(nbrs: &[usize]) -> usize {
    let mut crabs = nbrs.to_vec();
    crabs.sort_unstable();
    let middle = crabs.len() / 2;
    if crabs.len() % 2 == 0 {
        (crabs[middle - 1] + crabs[middle]) / 2
    } else {
        crabs[middle]
    }
}

fn get_mean(nbrs: &[usize]) -> usize {
    nbrs.iter().sum::<usize>() / nbrs.len()
}

struct InputData {
    crabs: Vec<usize>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            crabs: input.split(',').map(|n| n.parse().unwrap()).collect()
        }
    }

    fn get_scaling_cost(&self, cal_nbr: usize) -> usize {
        let scale = |x: usize| { x * (x + 1) / 2 };
        self.crabs.iter()
            .map(|crab|
                scale((*crab as i32 - cal_nbr as i32).unsigned_abs() as usize))
            .sum()
    }

    fn solve_part1(&self) -> usize {
        let cal_nbr = get_median(&self.crabs);
        self.crabs.iter()
            .map(|crab|
                (*crab as i32 - cal_nbr as i32).unsigned_abs() as usize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let distance = get_mean(&self.crabs);
        self.get_scaling_cost(distance).min(
            self.get_scaling_cost(distance - 1).min(
                self.get_scaling_cost(distance + 1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("16,1,2,0,4,2,7,1,2,14");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 37);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("16,1,2,0,4,2,7,1,2,14");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 168);
    }
}
