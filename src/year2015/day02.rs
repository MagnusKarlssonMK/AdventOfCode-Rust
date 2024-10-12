pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct InputData {
    gifts: Vec<(u32, u32, u32)>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        // Parse and store the dimensions sorted low to high - the order in the input
        // doesn't matter, so save some effort later by sorting it already here.
        fn parse_line(line: &str) -> (u32, u32, u32) {
            let mut sides = line
                .split('x')
                .map(|digit| digit.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            sides.sort_unstable();
            if sides.len() != 3 { (0, 0, 0) }
            else { (sides[0], sides[1], sides[2]) }
        }
        Self { gifts: input.lines().map(parse_line).collect() }
    }

    fn solve_part1(&self) -> u32 {
        self.gifts
            .iter()
            .map(|(l, w, h)|
                3 * (l * w) + 2 * h * (w + l)
            )
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        self.gifts
            .iter()
            .map(|(l, w, h)|
                2 * (l + w) + (l * w * h)
            )
            .sum()
    }
}
