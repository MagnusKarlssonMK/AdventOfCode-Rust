pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

impl Race {
    fn from(input: (usize, usize)) -> Self {
        Self {time: input.0, distance: input.1}
    }

    fn get_score(&self) -> usize {
        let min_velocity = ((self.time as f64 - ((self.time.pow(2) - (4 * self.distance)) as f64).sqrt()) / 2.0).floor() as usize + 1;
        let max_velocity = ((self.time as f64 + ((self.time.pow(2) - (4 * self.distance)) as f64).sqrt()) / 2.0).ceil() as usize - 1;
        1 + max_velocity - min_velocity
    }
}

struct InputData {
    races: Vec<Race>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let (times, distances) = input.split_once('\n').unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, distances) = distances.split_once(':').unwrap();
        Self {
            races: times
                .split_whitespace()
                .map(|v|v.parse().unwrap())
                .zip(distances
                    .split_whitespace()
                    .map(|v| v.parse().unwrap()))
                .map(Race::from)
                .collect()
        }
    }

    fn solve_part1(&self) -> usize {
        self.races.iter().map(|r| r.get_score()).product()
    }

    fn solve_part2(&self) -> usize {
        let megatime: Vec<String> = self.races.iter().map(|race| race.time.to_string()).collect();
        let megadistance: Vec<String> = self.races.iter().map(|race| race.distance.to_string()).collect();
        let megatime: usize = megatime.concat().parse().unwrap();
        let megadistance: usize = megadistance.concat().parse().unwrap();
        Race::from((megatime, megadistance)).get_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("Time:      7  15   30
Distance:  9  40  200");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 288);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("Time:      7  15   30
Distance:  9  40  200");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 71503);
    }
}
