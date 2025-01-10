//! # 2017 day 10 - Knot Hash
use std::{error::Error, fmt::Write};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1(256).to_string(),
        solution_data.solve_part2(256),
    ))
}

struct InputData<'a> {
    rawdata: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { rawdata: s })
    }
}

impl InputData<'_> {
    fn solve_part1(&self, buffer_len: usize) -> usize {
        let lengths: Vec<usize> = self
            .rawdata
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        let nbrs = generate_hash(&lengths, buffer_len, 1);
        nbrs.iter().take(2).product()
    }

    fn solve_part2(&self, buffer_len: usize) -> String {
        let mut lengths: Vec<usize> = self.rawdata.chars().map(|c| c as usize).collect();
        lengths.extend([17, 31, 73, 47, 23]);
        let sparse = generate_hash(&lengths, buffer_len, 64);
        let mut dense = String::new();

        for block in sparse.chunks_exact(16) {
            let n = block.iter().fold(0, |a, i| a ^ i);
            let _ = write!(&mut dense, "{n:02x}");
        }
        dense
    }
}

fn generate_hash(lengths: &[usize], buffersize: usize, rounds: usize) -> Vec<usize> {
    let mut nbrs: Vec<usize> = (0..buffersize).collect();
    let mut current_position = 0;
    let mut skipsize = 0;

    for _ in 0..rounds {
        for &length in lengths {
            let n = length + skipsize;
            nbrs[0..length].reverse();
            nbrs.rotate_left(n % buffersize);
            current_position += n;
            skipsize += 1;
        }
    }
    nbrs.rotate_right(current_position % buffersize);
    nbrs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "3,4,1,5";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(5), 12);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(
            solution_data.solve_part2(256),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
    }

    #[test]
    fn part2_example_2() {
        let testdata = "AoC 2017";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(
            solution_data.solve_part2(256),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
    }

    #[test]
    fn part2_example_3() {
        let testdata = "1,2,3";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(
            solution_data.solve_part2(256),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
    }

    #[test]
    fn part2_example_4() {
        let testdata = "1,2,4";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(
            solution_data.solve_part2(256),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
