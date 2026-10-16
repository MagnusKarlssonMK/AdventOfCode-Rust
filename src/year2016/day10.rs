//! # 2016 day 10 - Balance Bots
use crate::aoc_util::error::*;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve()?;
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Bot(u32),
    Output(u32),
}

#[derive(Clone, Copy, Debug)]
struct Delivery {
    destination: Destination,
    value: u32,
}

#[derive(Clone, Copy, Debug)]
struct Bot {
    low: Destination,
    high: Destination,
    holding: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
struct Targets(u32, u32);

#[derive(Debug)]
struct InputData {
    initial_deliveries: Vec<Delivery>,
    bots: HashMap<u32, Bot>,
    targets: Targets, // Have it defined here rather than consts so it can be modified for tests
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut initial_deliveries = Vec::new();
        let mut bots = HashMap::new();
        for line in s.lines() {
            let tokens: Vec<_> = line.split_ascii_whitespace().collect();
            match tokens.as_slice() {
                ["value", value, "goes", "to", "bot", to_bot] => {
                    initial_deliveries.push(Delivery {
                        destination: Destination::Bot(to_bot.parse()?),
                        value: value.parse()?,
                    });
                }
                [
                    "bot",
                    bot_id,
                    "gives",
                    "low",
                    "to",
                    low_kind,
                    low,
                    "and",
                    "high",
                    "to",
                    high_kind,
                    high,
                ] => {
                    let dest = |kind: &str, id: &str| -> Result<Destination, AocError> {
                        let id = id.parse()?;
                        match kind {
                            "bot" => Ok(Destination::Bot(id)),
                            "output" => Ok(Destination::Output(id)),
                            _ => Err(AocError::Invalid(format!("unknown target: {kind}"))),
                        }
                    };
                    bots.insert(
                        bot_id.parse()?,
                        Bot {
                            low: dest(low_kind, low)?,
                            high: dest(high_kind, high)?,
                            holding: None,
                        },
                    );
                }
                _ => return Err(AocError::Invalid(line.to_string())),
            }
        }
        Ok(Self {
            initial_deliveries,
            bots,
            targets: Targets(17, 61),
        })
    }
}

impl InputData {
    fn solve(&self) -> Result<(u32, u32), AocError> {
        let mut target_bot_nbr = None;
        let mut outputs: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut queue: VecDeque<Delivery> = self.initial_deliveries.iter().copied().collect();
        let mut bots = self.bots.clone();
        while let Some(delivery) = queue.pop_front() {
            match delivery.destination {
                Destination::Bot(to_bot) => {
                    if let Some(bot) = bots.get_mut(&to_bot) {
                        if let Some(stored_value) = bot.holding {
                            queue.push_back(Delivery {
                                destination: bot.low,
                                value: delivery.value.min(stored_value),
                            });
                            queue.push_back(Delivery {
                                destination: bot.high,
                                value: delivery.value.max(stored_value),
                            });
                            if [delivery.value, stored_value].contains(&self.targets.0)
                                && [delivery.value, stored_value].contains(&self.targets.1)
                            {
                                target_bot_nbr = Some(to_bot);
                            }
                            bot.holding = None;
                        } else {
                            bot.holding = Some(delivery.value);
                        }
                    } else {
                        return Err(AocError::Invalid(format!("unknown bot: {to_bot}")));
                    }
                }
                Destination::Output(v) => {
                    outputs.entry(v).or_default().push(delivery.value);
                }
            }
        }

        let p1 = target_bot_nbr.ok_or(AocError::NoSolution)?;
        let p2 = (0u32..3)
            .map(|i| {
                outputs
                    .get(&i)
                    .and_then(|chips| chips.first())
                    .copied()
                    .ok_or(AocError::NoSolution)
            })
            .product::<Result<u32, AocError>>()?;
        Ok((p1, p2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    const TEST_CUSTOM_1: &str = "value 61 goes to bot 4
value 17 goes to bot 4
value 1 goes to bot 0
value 2 goes to bot 0
value 3 goes to bot 1
value 4 goes to bot 1
value 5 goes to bot 2
value 6 goes to bot 3
bot 4 gives low to output 0 and high to output 1
bot 0 gives low to output 2 and high to output 3
bot 1 gives low to output 4 and high to output 5
bot 2 gives low to output 6 and high to output 7
bot 3 gives low to output 8 and high to output 9";

    const TEST_CUSTOM_2: &str = "value 1 goes to bot 0
value 2 goes to bot 0
value 4 goes to bot 1
value 3 goes to bot 1
value 6 goes to bot 2
value 5 goes to bot 2
bot 1 gives low to bot 0 and high to output 0
bot 2 gives low to bot 0 and high to output 2
bot 0 gives low to output 1 and high to output 0";

    #[test]
    fn part1_2_official_1() {
        let mut solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        solution_data.targets = Targets(2, 5);
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 2);
        assert_eq!(p2, 30);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_CUSTOM_1).unwrap();
        let (p1, p2) = solution_data.solve().unwrap();
        assert_eq!(p1, 4);
        assert_eq!(p2, 1037);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data = InputData::from_str(TEST_CUSTOM_2).unwrap();
        let err = solution_data.solve().unwrap_err();
        assert_eq!(err, AocError::NoSolution);
    }

    #[test]
    fn parser_value_instruction_ok() {
        let data = InputData::from_str("value 5 goes to bot 2").unwrap();
        assert_eq!(data.initial_deliveries.len(), 1);
        let delivery = data.initial_deliveries[0];
        assert_eq!(delivery.value, 5);
        assert!(matches!(delivery.destination, Destination::Bot(2)));
        assert!(data.bots.is_empty());
    }

    #[test]
    fn parser_bot_instruction_ok() {
        let data = InputData::from_str(
            "bot 0 gives low to output 1 and high to bot 2
bot 3 gives low to bot 4 and high to output 5",
        )
        .unwrap();
        assert_eq!(data.bots.len(), 2);

        let bot0 = data.bots.get(&0).unwrap();
        assert!(matches!(bot0.low, Destination::Output(1)));
        assert!(matches!(bot0.high, Destination::Bot(2)));
        assert_eq!(bot0.holding, None);

        let bot3 = data.bots.get(&3).unwrap();
        assert!(matches!(bot3.low, Destination::Bot(4)));
        assert!(matches!(bot3.high, Destination::Output(5)));
        assert_eq!(bot3.holding, None);

        assert!(data.initial_deliveries.is_empty());
    }

    #[test]
    fn parser_parse_int_error() {
        let err = InputData::from_str("value x5 goes to bot 2").unwrap_err();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn parser_unknown_target_kind() {
        let err = InputData::from_str("bot 0 gives low to bin 1 and high to bot 2").unwrap_err();
        assert_eq!(err, AocError::Invalid("unknown target: bin".to_string()));
    }

    #[test]
    fn parser_invalid_line() {
        let err =
            InputData::from_str("value 5 goes to bot 2\nnot a valid instruction").unwrap_err();
        assert_eq!(
            err,
            AocError::Invalid("not a valid instruction".to_string())
        );
    }
}
