pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct InputData {
    numbers: Vec<Vec<usize>>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            numbers: input.lines()
                .map(|line|
                    line.split_whitespace()
                        .filter_map(|n| n.trim_end_matches(',').parse().ok())
                        .skip(1)
                        .collect())
                .collect()
        }
    }

    fn solve(&self) -> (usize, usize) {
        let size = self.numbers.len();
        let mut seen = vec![false; size];
        let mut program_groups = Vec::new();
        (0..size).for_each(|i| if !seen[i] {
            seen[i] = true;
            program_groups.push(self.search_programs(&mut seen, i));
        });
        (program_groups[0], program_groups.len())
    }

    fn search_programs(&self, seen: &mut [bool], index: usize) -> usize {
        let mut total = 1;
        for program_id in self.numbers[index].iter() {
            if !seen[*program_id] {
                seen[*program_id] = true;
                total += self.search_programs(seen, *program_id);
            }
        }
        total
    }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 6);
        assert_eq!(p2, 2);
    }
}
