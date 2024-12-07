pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(PartialEq)]
enum CalibrationResult {
    Ok(usize),
    ConcatinatedOk(usize),
    NotOk
}

struct Equation {
    test_value: usize,
    numbers: Vec<usize>
}

impl Equation {
    fn parse_str(input: &str) -> Self {
        let (left, right) = input.split_once(": ").unwrap();
        Self {
            test_value: left.parse().unwrap(),
            numbers: right.split_whitespace().map(|n| n.parse().unwrap()).collect()
        }
    }

    fn calibrate(&self) -> CalibrationResult {
        self.validate(self.numbers[0], &self.numbers[1..])
    }

    fn validate(&self, total: usize, nbrs: &[usize]) -> CalibrationResult {
        if nbrs.is_empty() {
            if total == self.test_value {
                CalibrationResult::Ok(total)
            } else {
                CalibrationResult::NotOk
            }
        } else if total > self.test_value {
            CalibrationResult::NotOk
        } else {
            // Prioritize Ok result, only start concatenating if necessary
            let add_result = self.validate(total + nbrs[0], &nbrs[1..]);
            if let CalibrationResult::Ok(_) = add_result {
                return add_result;
            }
            let mul_result = self.validate(total * nbrs[0], &nbrs[1..]);
            if let CalibrationResult::Ok(_) = mul_result {
                return mul_result;
            }
            if let CalibrationResult::ConcatinatedOk(_) = add_result {
                return add_result;
            } else if let CalibrationResult::ConcatinatedOk(_) = mul_result {
                return mul_result;
            } else {
                let conc_result = self.validate(total * concatinate(nbrs[0]) + nbrs[0], &nbrs[1..]);
                match conc_result {
                    CalibrationResult::Ok(v) => return CalibrationResult::ConcatinatedOk(v),
                    CalibrationResult::ConcatinatedOk(v) => return CalibrationResult::ConcatinatedOk(v),
                    CalibrationResult::NotOk => ()
                }
            }
            CalibrationResult::NotOk
        }
    }
}

fn concatinate(right: usize) -> usize {
    let mut multiplier = 10;
    while multiplier <= right {
        multiplier *= 10;
    }
    multiplier
}


struct InputData {
    equations: Vec<Equation>
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            equations: input.lines().map(Equation::parse_str).collect()
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        for eq in &self.equations {
            match eq.calibrate() {
                CalibrationResult::Ok(v) => {p1 += v; p2 += v;},
                CalibrationResult::ConcatinatedOk(v) => p2 += v,
                CalibrationResult::NotOk => ()
            }
        }
        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3749);
        assert_eq!(p2, 11387);
    }
}
