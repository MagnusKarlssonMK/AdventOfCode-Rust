//! # 2023 day 6 - Wait For It
//!
//! Uses quadratic formula to calculate the two points where the score intersects the old record.
//! The biggest challenge is to round the points off in the right direction, to also account for
//! the few cases where the solution was exactly the same value as the old record. For part 1,
//! the answer is the product of the number of hold times that win each of the races. For part 2,
//! the digits of all the times are concatenated into one big time, and the same is done with all
//! the distances, which turns the whole input into a single race.
use crate::aoc_util::error::{AocError, OptionExt};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_score(&self) -> u64 {
        let time = u128::from(self.time);
        let distance = u128::from(self.distance);
        let a = time * time;
        let b = 4 * distance;
        if b >= a {
            // Impossible to beat the record
            return 0;
        }
        let root = (a - b).isqrt();
        let mut min_holdtime = (time - root).div_ceil(2);
        let mut max_holdtime = time.midpoint(root);

        // Adjust rounding if necessary
        if min_holdtime * (time - min_holdtime) > distance {
            min_holdtime -= 1;
        }
        if max_holdtime * (time - max_holdtime) > distance {
            max_holdtime += 1;
        }

        // The score is at most `time`, so this conversion is exact.
        (max_holdtime - min_holdtime - 1) as u64
    }
}

struct InputData {
    races: Vec<Race>,
    mega_race: Race,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time_line = lines
            .next()
            .ctx("time line missing")?
            .strip_prefix("Time:")
            .ctx("time line prefix")?;
        let distance_line = lines
            .next()
            .ctx("distance line missing")?
            .strip_prefix("Distance:")
            .ctx("distance line prefix")?;

        let times: Vec<u64> = time_line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let distances: Vec<u64> = distance_line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        if times.is_empty() || times.len() != distances.len() {
            return Err(AocError::Invalid(
                "not same number of times and distances".to_string(),
            ));
        }
        Ok(Self {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
            mega_race: Race {
                time: time_line.split_whitespace().collect::<String>().parse()?,
                distance: distance_line
                    .split_whitespace()
                    .collect::<String>()
                    .parse()?,
            },
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> u64 {
        self.races.iter().map(|r| r.get_score()).product()
    }

    fn solve_part2(&self) -> u64 {
        self.mega_race.get_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "Time:      7  15   30
Distance:  9  40  200";

    const TEST_CUSTOM_1: &str = "Time: 4
Distance: 4";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 288);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), 71503);
    }

    #[test]
    fn part1_custom_1() {
        // t * (4 - t) never exceeds 4, so there is no winning hold time.
        let data: InputData = TEST_CUSTOM_1.parse().unwrap();
        assert_eq!(data.solve_part1(), 0);
    }
}
