//! # 2017 day 8 - I Heard You Like Registers
use std::{collections::HashMap, error::Error};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

struct InputData<'a> {
    instructions: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { instructions: s })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (isize, isize) {
        let mut registers = HashMap::new();
        let mut maxval = 0;
        for instr in self.instructions.lines() {
            let mut parts = instr.split_whitespace();
            let reg = parts.next().unwrap();
            let op = parts.next().unwrap();
            let val: isize = parts.next().unwrap().parse().unwrap();
            parts.next();
            let cnd = parts.next().unwrap();
            let cmp = parts.next().unwrap();
            let cnd_val: isize = parts.next().unwrap().parse().unwrap();

            let cnd_reg = registers.entry(cnd).or_insert(0);

            let condition_met = match cmp {
                ">" => *cnd_reg > cnd_val,
                ">=" => *cnd_reg >= cnd_val,
                "<" => *cnd_reg < cnd_val,
                "<=" => *cnd_reg <= cnd_val,
                "==" => *cnd_reg == cnd_val,
                "!=" => *cnd_reg != cnd_val,
                _ => unreachable!(),
            };

            if condition_met {
                let reg_val = registers.entry(reg).or_insert(0);
                match op {
                    "inc" => *reg_val += val,
                    "dec" => *reg_val -= val,
                    _ => unreachable!(),
                }
                maxval = maxval.max(*reg_val);
            };
        }
        (*registers.values().max().unwrap(), maxval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_1_2_example_1() {
        let testdata = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 1);
        assert_eq!(p2, 10);
    }
}
