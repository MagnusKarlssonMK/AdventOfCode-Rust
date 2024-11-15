pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct Hand {
    red: usize,
    blue: usize,
    green: usize,
}

impl Hand {
    fn parse(input: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        let colors = input.split(", ");
        for n in colors {
            let (nbr, color) = n.split_once(' ').unwrap();
            match color.chars().nth(0).unwrap() {
                'r' => red += nbr.parse::<usize>().unwrap(),
                'b' => blue += nbr.parse::<usize>().unwrap(),
                'g' => green += nbr.parse::<usize>().unwrap(),
                _ => unreachable!()
            }
        }
        Self { red, green, blue }
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.blue <= 14 && self.green <= 13
    }

    fn get_power(&self) -> usize {
        self.red * self.blue * self.green
    }

    fn get_max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green) }
    }
}

struct Game {
    game_id: usize,
    hands: Vec<Hand>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let (left, right) = input.split_once(": ").unwrap();
        let (_, gid) = left.split_once(' ').unwrap();
        Self {
            game_id: gid.parse().unwrap(),
            hands: right.split("; ").map(Hand::parse).collect() }
    }

    fn is_valid(&self) -> bool {
        self.hands.iter().all(|h| h.is_valid())
    }

    fn get_power(&self) -> usize {
        let mut minimum_required = Hand {red: 0, blue: 0, green: 0};
        for h in self.hands.iter() {
            minimum_required = h.get_max(&minimum_required);
        }
        minimum_required.get_power()
    }
}

struct InputData {
    games: Vec<Game>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            games: input.lines().map(Game::parse).collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.games.iter()
            .filter(|g| g.is_valid())
            .map(|g| g.game_id)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.games.iter().map(|g| g.get_power()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("1");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("2");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 2);
    }
}
