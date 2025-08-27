//! # 2023 day 15 - Lens Library
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

fn hash_algorithm(input: &str) -> usize {
    input
        .chars()
        .fold(0, |h, c| ((h + u32::from(c)) * 17) % 256) as usize
}

struct Lens<'b> {
    label: &'b str,
    strength: usize,
}

struct InputData<'a> {
    steps: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            steps: s.split(",").collect(),
        })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        self.steps.iter().map(|w| hash_algorithm(w)).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut boxes: Vec<Vec<Lens<'_>>> = Vec::new();
        boxes.resize_with(256, Vec::new);

        for step in self.steps.iter() {
            if let Some((label, strength)) = step.split_once('=') {
                let strength: usize = strength.parse().unwrap();
                let hashed = hash_algorithm(label);
                if let Some(idx) = boxes[hashed].iter().position(|lens| lens.label == label) {
                    boxes[hashed][idx].strength = strength;
                } else {
                    boxes[hashed].push(Lens { label, strength });
                }
            } else {
                let label = step.strip_suffix('-').unwrap();
                let hashed = hash_algorithm(label);
                if let Some(idx) = boxes[hashed].iter().position(|lens| lens.label == label) {
                    boxes[hashed].remove(idx);
                }
            }
        }

        boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, lens)| (i + 1) * (j + 1) * lens.strength)
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_example_1() {
        let testdata = "HASH";
        assert_eq!(hash_algorithm(&testdata), 52);
    }

    #[test]
    fn part1_example_2() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part1(), 1320);
    }

    #[test]
    fn part2_example_1() {
        let solution_data = InputData::try_from(TEST_DATA).unwrap();
        assert_eq!(solution_data.solve_part2(), 145);
    }
}
