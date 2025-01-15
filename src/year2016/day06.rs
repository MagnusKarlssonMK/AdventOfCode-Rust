//! # 2016 day 6 - Signals and Noise
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok(solution_data.solve())
}

struct InputData<'a> {
    codes: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            codes: s.lines().collect(),
        })
    }
}

impl InputData<'_> {
    fn solve(&self) -> (String, String) {
        let width = self.codes.first().unwrap().len();
        let mut counter = vec![vec![0_usize; (1 + b'z' - b'a') as usize]; width];
        for code in &self.codes {
            for (i, c) in code.bytes().enumerate() {
                counter[i][(c - b'a') as usize] += 1;
            }
        }
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        for count in &counter {
            let mut a: Vec<(usize, u8)> = count
                .iter()
                .enumerate()
                .filter(|(_, n)| **n > 0)
                .map(|(i, n)| (*n, i as u8))
                .collect();
            a.sort();
            p1.push((a.last().unwrap().1 + b'a') as char);
            p2.push((a.first().unwrap().1 + b'a') as char);
        }
        (p1.iter().collect(), p2.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        let solution_data = InputData::try_from(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "easter");
        assert_eq!(p2, "advent");
    }
}
