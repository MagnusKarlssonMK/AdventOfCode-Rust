//! # 2024 day 3 - Mull It Over
use crate::aoc_util::error::AocError;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data = InputData::try_from(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    program: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = AocError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { program: s })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        calculate(self.program)
    }

    fn solve_part2(&self) -> usize {
        self.program
            .split("do()")
            .map(|block| {
                if let Some((do_block, _)) = block.split_once("don't()") {
                    calculate(do_block)
                } else {
                    calculate(block)
                }
            })
            .sum()
    }
}

fn calculate(input: &str) -> usize {
    input
        .split("mul(")
        .map(|part| {
            if let Some((p, _)) = part.split_once(')')
                && let Some((n1, n2)) = p.split_once(',')
            {
                let nbr1: usize = n1.parse().unwrap_or_default();
                let nbr2: usize = n2.parse().unwrap_or_default();
                nbr1 * nbr2
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_OFFICIAL_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 161);
    }

    #[test]
    fn part2_official_2() {
        let solution_data = InputData::try_from(TEST_OFFICIAL_2).unwrap();
        assert_eq!(solution_data.solve_part2(), 48);
    }
}
