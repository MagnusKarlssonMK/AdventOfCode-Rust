//! # 2023 day 20 - Pulse Propagation
//!
//! It would probably make more sense to model this with a Module enum instead, but I wanted to
//! take the opportunity to mess around with the trait and dyn box system.
use crate::aoc_util::math::lcm;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut solution_data = InputData::from_str(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Signal {
    receiver: String,
    sender: String,
    pulse: Pulse,
}

trait ProcessSignal {
    fn handle_pulse(&mut self, m: &Signal) -> Vec<Signal>;

    fn get_outputs(&self) -> Vec<String>;

    fn reset(&mut self) {}
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    outputs: Vec<String>,
    is_on: bool,
}

impl ProcessSignal for FlipFlop {
    fn handle_pulse(&mut self, m: &Signal) -> Vec<Signal> {
        if m.pulse == Pulse::Low {
            self.is_on = !self.is_on;
            let pulse = if self.is_on { Pulse::High } else { Pulse::Low };
            self.outputs
                .iter()
                .map(|o| Signal {
                    receiver: o.to_string(),
                    sender: self.name.to_string(),
                    pulse,
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }

    fn reset(&mut self) {
        self.is_on = false;
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    outputs: Vec<String>,
    inputs: HashMap<String, Pulse>,
}

impl ProcessSignal for Conjunction {
    fn handle_pulse(&mut self, m: &Signal) -> Vec<Signal> {
        self.inputs.insert(m.sender.clone(), m.pulse);
        let pulse = if self.inputs.values().all(|v| *v == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };
        self.outputs
            .iter()
            .map(|o| Signal {
                receiver: o.to_string(),
                sender: self.name.to_string(),
                pulse,
            })
            .collect()
    }

    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }

    fn reset(&mut self) {
        for p in self.inputs.values_mut() {
            *p = Pulse::Low;
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    outputs: Vec<String>,
}

impl ProcessSignal for Broadcaster {
    fn handle_pulse(&mut self, m: &Signal) -> Vec<Signal> {
        self.outputs
            .iter()
            .map(|o| Signal {
                receiver: o.to_string(),
                sender: self.name.to_string(),
                pulse: m.pulse,
            })
            .collect()
    }

    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

struct InputData {
    network: HashMap<String, Box<dyn ProcessSignal>>,
    rx_input: String,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut network: HashMap<String, Box<dyn ProcessSignal>> = HashMap::new();
        let mut conjunctions = HashMap::new();
        let mut rx_input = "".to_string();
        for line in s.lines() {
            let (left, right) = line.split_once(" -> ").unwrap();
            match left.chars().nth(0).unwrap() {
                '%' => {
                    // Flip Flop
                    network.insert(
                        left[1..].to_string(),
                        Box::new(FlipFlop {
                            name: left[1..].to_string(),
                            outputs: right.split(", ").map(|s| s.to_string()).collect(),
                            is_on: false,
                        }),
                    );
                }
                '&' => {
                    // Conjunction
                    network.insert(
                        left[1..].to_string(),
                        Box::new(Conjunction {
                            name: left[1..].to_string(),
                            outputs: right.split(", ").map(|s| s.to_string()).collect(),
                            inputs: HashMap::new(),
                        }),
                    );
                    conjunctions.insert(left[1..].to_string(), vec![]);
                }
                _ => {
                    // Broadcaster
                    network.insert(
                        left[0..].to_string(),
                        Box::new(Broadcaster {
                            name: left[0..].to_string(),
                            outputs: right.split(", ").map(|s| s.to_string()).collect(),
                        }),
                    );
                }
            }
        }

        for (n, m) in &network {
            for o in &m.get_outputs() {
                if o == "rx" {
                    rx_input = n.to_string();
                }
                if conjunctions.contains_key(o) {
                    conjunctions
                        .entry(o.to_string())
                        .and_modify(|e| e.push(n.to_string()));
                }
            }
        }
        for (n, inputs) in &conjunctions {
            for i in inputs {
                // Add the input to the conjunction's memory by inserting a low pulse and ignoring the returned output
                network.get_mut(n).unwrap().handle_pulse(&Signal {
                    receiver: n.to_string(),
                    sender: i.to_string(),
                    pulse: Pulse::Low,
                });
            }
        }
        Ok(Self { network, rx_input })
    }
}

impl InputData {
    fn solve_part1(&mut self) -> usize {
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let mut msg_queue = VecDeque::from([Signal {
                receiver: "broadcaster".to_string(),
                sender: "".to_string(),
                pulse: Pulse::Low,
            }]);
            while let Some(msg) = msg_queue.pop_front() {
                if msg.pulse == Pulse::High {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
                // Note - "rx" exists as receiver in the input, but not as sender, i.e. get_mut can return None
                if let Some(module) = self.network.get_mut(&msg.receiver) {
                    for new_msg in module.handle_pulse(&msg) {
                        msg_queue.push_back(new_msg);
                    }
                }
            }
        }
        low_count * high_count
    }

    fn solve_part2(&mut self) -> usize {
        // Reset all modules while also finding the modules connecting to the node connecting to "rx"
        let mut rx_input_inputs: HashMap<String, usize> = HashMap::new();
        for (name, module) in self.network.iter_mut() {
            module.reset();
            for o in &module.get_outputs() {
                if *o == self.rx_input {
                    rx_input_inputs.insert(name.to_string(), 0);
                }
            }
        }

        // Keep pushing the button until all the modules connecting to the node connecting to "rx" has received a high signal
        // and record the number of button pushes for each
        let mut push_count = 0;
        while rx_input_inputs.values().any(|v| *v == 0) {
            let mut msg_queue = VecDeque::from([Signal {
                receiver: "broadcaster".to_string(),
                sender: "".to_string(),
                pulse: Pulse::Low,
            }]);
            push_count += 1;
            while let Some(msg) = msg_queue.pop_front() {
                // Note - "rx" exists as receiver in the input, but not as sender, i.e. get_mut can return None
                if let Some(module) = self.network.get_mut(&msg.receiver) {
                    for new_msg in module.handle_pulse(&msg) {
                        if rx_input_inputs.contains_key(&msg.receiver)
                            && *rx_input_inputs.get(&msg.receiver).unwrap() == 0
                            && new_msg.pulse == Pulse::High
                        {
                            rx_input_inputs
                                .entry(msg.receiver.to_string())
                                .and_modify(|v| *v = push_count);
                        }
                        msg_queue.push_back(new_msg);
                    }
                }
            }
        }

        rx_input_inputs.values().fold(1, |acc, v| lcm(acc, *v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let mut solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 32000000);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let mut solution_data = InputData::from_str(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 11687500);
    }
}
