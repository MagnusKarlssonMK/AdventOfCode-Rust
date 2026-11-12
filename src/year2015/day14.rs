//! # 2015 day 14 - Reindeer Olympics
use crate::aoc_util::error::*;
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1(TIME_LIMIT).to_string(),
        solution_data.solve_part2(TIME_LIMIT).to_string(),
    ))
}

/// The official time limit
const TIME_LIMIT: u32 = 2503;

#[derive(Debug)]
struct Reindeer {
    /// The flying speed in km/s
    fly_speed: u32,
    /// The duration in seconds that the reindeer can fly
    fly_time: u32,
    /// The duration in seconds that the reindeer must rest
    rest_time: u32,
}

impl FromStr for Reindeer {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            [
                _,
                "can",
                "fly",
                s,
                "km/s",
                "for",
                t,
                _,
                "but",
                "then",
                "must",
                "rest",
                "for",
                r,
                _,
            ] => {
                let fly_speed = s.parse()?;
                let fly_time = t.parse()?;
                let rest_time = r.parse()?;
                if fly_time == 0 && rest_time == 0 {
                    Err(AocError::Invalid(format!(
                        "both fly time and rest time zero: {s}"
                    )))
                } else {
                    Ok(Self {
                        fly_speed,
                        fly_time,
                        rest_time,
                    })
                }
            }
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Reindeer {
    fn get_distance(&self, duration: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = duration / cycle_time;
        let remainder = duration % cycle_time;
        self.fly_speed * ((cycles * self.fly_time) + self.fly_time.min(remainder))
    }
}

#[derive(Debug)]
struct InputData {
    reindeer: Vec<Reindeer>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            reindeer: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self, duration: u32) -> u32 {
        self.reindeer
            .iter()
            .map(|r| r.get_distance(duration))
            .max()
            .expect("reindeer cannot be empty")
    }

    fn solve_part2(&self, duration: u32) -> u32 {
        let mut scores = vec![0u32; self.reindeer.len()];
        for seconds in 1..=duration {
            let mut leader_distance = 0;
            let mut leaders = Vec::new();
            for (i, r) in self.reindeer.iter().enumerate() {
                let new_dist = r.get_distance(seconds);
                if new_dist > leader_distance {
                    leaders = vec![i];
                    leader_distance = new_dist;
                } else if new_dist == leader_distance {
                    leaders.push(i);
                }
            }
            for lead in leaders {
                scores[lead] += 1;
            }
        }
        scores.into_iter().max().expect("reindeer cannot be empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    /// A comes first deliberately. A and B share the lead at 29 of the 2503 seconds, and A is the
    /// one who wins on points, so loosening the lead test to >= would let B take those 29 points
    /// off A and drop the answer to 2335. With A listed last the same loosening only costs B
    /// points, which nobody reports.
    const TEST_CUSTOM_1: &str = "A can fly 7 km/s for 20 seconds, but then must rest for 3 seconds.
B can fly 20 km/s for 3 seconds, but then must rest for 7 seconds.
C can fly 10 km/s for 5 seconds, but then must rest for 5 seconds.";

    /// Invalid input since both fly time and rest time are zero
    const TEST_CUSTOM_2: &str =
        "Comet can fly 14 km/s for 0 seconds, but then must rest for 0 seconds.";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(1000), 1120);
    }

    #[test]
    fn part1_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(TIME_LIMIT), 15253);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(1000), 689);
    }

    #[test]
    fn part2_custom_1() {
        let solution_data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(TIME_LIMIT), 2364);
    }

    #[test]
    fn parser_invalid_input() {
        let err = TEST_CUSTOM_2.parse::<InputData>().unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)));
    }
}
