//! # 2023 day 6 - Wait For It
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn from(input: (usize, usize)) -> Self {
        Self {
            time: input.0,
            distance: input.1,
        }
    }

    fn get_score(&self) -> usize {
        let min_velocity =
            ((self.time as f64 - ((self.time.pow(2) - (4 * self.distance)) as f64).sqrt()) / 2.0)
                .floor() as usize
                + 1;
        let max_velocity =
            ((self.time as f64 + ((self.time.pow(2) - (4 * self.distance)) as f64).sqrt()) / 2.0)
                .ceil() as usize
                - 1;
        1 + max_velocity - min_velocity
    }
}

struct InputData {
    races: Vec<Race>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (times, distances) = s.split_once('\n').unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, distances) = distances.split_once(':').unwrap();
        Ok(Self {
            races: times
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .zip(distances.split_whitespace().map(|v| v.parse().unwrap()))
                .map(Race::from)
                .collect(),
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        self.races.iter().map(|r| r.get_score()).product()
    }

    fn solve_part2(&self) -> usize {
        let megatime: Vec<String> = self
            .races
            .iter()
            .map(|race| race.time.to_string())
            .collect();
        let megadistance: Vec<String> = self
            .races
            .iter()
            .map(|race| race.distance.to_string())
            .collect();
        let megatime: usize = megatime.concat().parse().unwrap();
        let megadistance: usize = megadistance.concat().parse().unwrap();
        Race::from((megatime, megadistance)).get_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 288);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::from_str(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 71503);
    }
}
