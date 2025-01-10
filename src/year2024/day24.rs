//! # 2024 day 24 - Crossed Wires
//!
//! ## Part 1
//!
//! Uses the BD algorithm (Brain Dead) - put all instructions in a queue
//! and cycle through it, gradually ripping out the items that have both
//! inputs mapped, and keep doing that until the queue is empty. Then
//! extract the Z-items from all the mapped numbers and bitshift into a
//! number. It... works.
//!
//! ## Part 2
//!
//! BD approach kind of got an anwer in the end, but nothing I'd like to
//! keep around. So instead resorting to reading up on Ripple Carry Adder
//! to work out something simpler:
//!
//! - All correct AND gates except for the first one will be connected to
//!   an OR gate.
//! - Only XOR gates can connect to output, except for an OR gate for the
//!   last one.
//! - A correct OR gate can never connect to another OR gate
//! - Except for the first XOR, a correct XOR will connect to another XOR,
//!   which in turn connects to output.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2(),
    ))
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

impl Operation {
    const fn process(&self, a: usize, b: usize) -> usize {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

struct Gate<'b> {
    in_a: &'b str,
    in_b: &'b str,
    out: &'b str,
    op: Operation,
}

impl<'b> TryFrom<&'b str> for Gate<'b> {
    type Error = ();
    fn try_from(s: &'b str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_whitespace();
        let in_a = tokens.next().unwrap();
        let op = Operation::from_str(tokens.next().unwrap()).unwrap();
        let in_b = tokens.next().unwrap();
        tokens.next();
        let out = tokens.next().unwrap();
        Ok(Self {
            in_a,
            in_b,
            out,
            op,
        })
    }
}

struct InputData<'a> {
    initial_values: Vec<(&'a str, usize)>,
    gates: Vec<Gate<'a>>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (iv, g) = s.split_once("\n\n").unwrap();
        let initial_values = iv
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(": ").unwrap();
                (left, right.parse::<usize>().unwrap())
            })
            .collect();
        let gates = g
            .lines()
            .map(|line| Gate::try_from(line).unwrap())
            .collect();
        Ok(Self {
            initial_values,
            gates,
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        let mut wires: HashMap<&str, usize> = HashMap::new();
        self.initial_values.iter().for_each(|(iv_name, iv_val)| {
            wires.insert(iv_name, *iv_val);
        });
        let mut queue = VecDeque::new();
        self.gates.iter().for_each(|gate| {
            queue.push_back(gate);
        });
        while let Some(g) = queue.pop_front() {
            if !wires.contains_key(g.in_a) || !wires.contains_key(g.in_b) {
                queue.push_back(g);
            } else {
                wires.insert(g.out, g.op.process(wires[g.in_a], wires[g.in_b]));
            }
        }
        let mut z = 0;
        for (wire, val) in wires {
            if let Some(rest) = wire.strip_prefix('z') {
                if let Ok(nbr) = rest.parse::<usize>() {
                    z |= val << nbr;
                }
            }
        }
        z
    }

    fn solve_part2(&self) -> String {
        let mut input_types = HashSet::new();
        let mut max_z = "z00";
        for gate in self.gates.iter() {
            input_types.insert((gate.in_a, &gate.op));
            input_types.insert((gate.in_b, &gate.op));
            if gate.out > max_z {
                max_z = gate.out;
            }
        }

        let mut swapped_outputs = Vec::new();
        for gate in self.gates.iter() {
            match gate.op {
                Operation::And => {
                    if gate.in_a != "x00"
                        && gate.in_b != "x00"
                        && !input_types.contains(&(gate.out, &Operation::Or))
                    {
                        swapped_outputs.push(gate.out);
                    }
                }
                Operation::Or => {
                    if (gate.out.starts_with('z') && gate.out != max_z)
                        || input_types.contains(&(gate.out, &Operation::Or))
                    {
                        swapped_outputs.push(gate.out);
                    }
                }
                Operation::Xor => {
                    if gate.in_a.starts_with('x') || gate.in_b.starts_with('x') {
                        if gate.in_a != "x00"
                            && gate.in_b != "x00"
                            && !input_types.contains(&(gate.out, &Operation::Xor))
                        {
                            swapped_outputs.push(gate.out);
                        }
                    } else if !gate.out.starts_with('z') {
                        swapped_outputs.push(gate.out);
                    }
                }
            }
        }
        swapped_outputs.sort_unstable();
        swapped_outputs.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 4);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 2024);
    }
}
