#![allow(unused)]  // Remove me!!!
use std::collections::HashMap;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(&input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

#[derive(Debug)]
struct Program {
    weight: usize,
    leafs: Vec<String>
}

impl Program {
    fn from(line: &str) -> Self {
        let tokens: Vec<String> = line.split_whitespace().map(|token| token.to_string()).collect();
        let mut leaflist: Vec<String> = Vec::new();
        if tokens.len() > 2 {
            for idx in 2..tokens.len() {
                leaflist.push(tokens[idx].trim_end_matches(',').to_string());
            }
        }
        Self { weight: tokens[0].strip_prefix('(').unwrap().strip_suffix(')').unwrap().parse().unwrap(),
            leafs: leaflist }
    }
}

struct InputData {
    programs: HashMap<String, Program>,
    parents: HashMap<String, String>,
    root: String
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let name_and_prog_list: Vec<(String, Program)> = input.lines()
            .map(|line| {
                let (name, rest) = line.split_once(' ').unwrap();
                (name.to_string(), Program::from(rest))
            }).collect();
        Self {
            programs: HashMap::from_iter(name_and_prog_list),
            parents: HashMap::new(),
            root: String::from("ROOT") }

        // Todo: fill in parents and root (root is answer to part 1)
        // Then onto part 2
        // Remove compiler warning suppression
    }

    fn solve_part1(&self) -> String {
        println!("HUH: {:?}", self.programs["aaeeqh"]);
        self.root.clone()
    }

    fn solve_part2(&self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_1() {
        let testdata = String::from("pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), String::from("tknk"));
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from("pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 10);
    }
}
