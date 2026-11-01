//! # 2018 day 4 - Repose Record
//!
//! Parse chronological guard shift records to track when each guard falls asleep and
//! wakes up. Build a minute-by-minute sleep count per guard. Part 1 finds the guard
//! with the most total minutes asleep and returns their ID times the minute they
//! sleep most. Part 2 finds the guard who sleeps most frequently on a single minute
//! and returns their ID times that minute.
use crate::aoc_util::error::*;
use std::{collections::BTreeMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    BeginShift,
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Record {
    event: Event,
    guard: usize,
    minute: usize,
}

struct InputData {
    records: Vec<Record>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_guard = 0usize;
        let mut records = Vec::new();
        // Parse the lines in time order
        let mut lines: Vec<_> = s.lines().collect();
        lines.sort_unstable();
        for line in &lines {
            let (timestr, eventstr) = line.split_once("] ").ctx("separator")?;
            let (_, minute) = timestr.split_once(':').ctx("minute separator")?;
            let minute: usize = minute.parse()?;
            let event = match eventstr.split(' ').collect::<Vec<_>>().as_slice() {
                ["Guard", g, "begins", "shift"] => {
                    current_guard = g.trim_start_matches('#').parse()?;
                    Event::BeginShift
                }
                ["wakes", "up"] => Event::WakeUp,
                ["falls", "asleep"] => Event::FallAsleep,
                _ => {
                    return Err(AocError::Invalid(line.to_string()));
                }
            };
            records.push(Record {
                event,
                guard: current_guard,
                minute,
            });
        }
        Ok(Self { records })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        let mut guards: BTreeMap<usize, BTreeMap<usize, usize>> = self
            .records
            .iter()
            .filter(|r| r.event == Event::BeginShift)
            .map(|r| (r.guard, BTreeMap::new()))
            .collect();

        let mut asleep_minute = 0;
        for r in &self.records {
            match r.event {
                Event::BeginShift => (),
                Event::FallAsleep => asleep_minute = r.minute,
                Event::WakeUp => {
                    for t in asleep_minute..r.minute {
                        if let Some(map) = guards.get_mut(&r.guard) {
                            *map.entry(t).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        let mut max_asleep = (0usize, 0usize, 0usize);
        let mut max_minute = (0usize, 0usize, 0usize);

        for (g, v) in guards.iter() {
            let m: usize = v.values().sum();
            if m == 0 {
                continue;
            }
            let best_min_total = v
                .iter()
                .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
                .unwrap()
                .0;
            if m > max_asleep.1 {
                max_asleep = (*g, m, *best_min_total);
            }
            let max_count = *v.values().max().unwrap_or(&0);
            if max_count == 0 {
                continue;
            }
            let best_min_freq = v
                .iter()
                .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
                .unwrap()
                .0;
            if max_count > max_minute.1 {
                max_minute = (*g, max_count, *best_min_freq);
            }
        }
        (max_asleep.0 * max_asleep.2, max_minute.0 * max_minute.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    /// Guards sleep the same 20 minutes, but across different months and years, to
    /// exercise the chronological ordering (year, then month comparisons).
    const TEST_CUSTOM_1: &str = "[1518-02-01 00:00] Guard #1 begins shift
[1518-02-01 00:05] falls asleep
[1518-02-01 00:25] wakes up
[1518-03-01 00:00] Guard #2 begins shift
[1518-03-01 00:05] falls asleep
[1518-03-01 00:25] wakes up
[1519-02-01 00:00] Guard #3 begins shift
[1519-02-01 00:05] falls asleep
[1519-02-01 00:25] wakes up";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 240);
        assert_eq!(p2, 4455);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 5);
        assert_eq!(p2, 5);
    }
}
