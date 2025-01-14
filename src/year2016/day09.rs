//! # 2016 day 9 - Explosives in Cyberspace
use std::error::Error;

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::try_from(input).unwrap();
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

struct InputData<'a> {
    filedata: &'a str,
}

impl<'a> TryFrom<&'a str> for InputData<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self { filedata: s })
    }
}

impl InputData<'_> {
    fn solve_part1(&self) -> usize {
        get_decompressed_len(self.filedata, false)
    }

    fn solve_part2(&self) -> usize {
        get_decompressed_len(self.filedata, true)
    }
}

fn get_decompressed_len(filedata: &str, recurse: bool) -> usize {
    let mut fp = 0;
    let mut total = 0;
    while let Some(c) = filedata.chars().nth(fp) {
        match c {
            '(' => {
                fp += 1;
                let mut marker = Vec::new();
                while let Some(d) = filedata.chars().nth(fp) {
                    if d != ')' {
                        marker.push(d);
                        fp += 1;
                    } else {
                        break;
                    }
                }
                fp += 1;
                let m = marker.iter().collect::<String>();
                let (a, b) = m.split_once('x').unwrap();
                let a: usize = a.parse().unwrap();
                let b: usize = b.parse().unwrap();
                if recurse {
                    total += b * get_decompressed_len(&filedata[fp..fp + a], true);
                } else {
                    total += a * b;
                }
                fp += a;
            }
            _ => {
                fp += 1;
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "ADVENT";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part1_example_2() {
        let testdata = "A(1x5)BC";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part1_example_3() {
        let testdata = "(3x3)XYZ";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 9);
    }

    #[test]
    fn part1_example_4() {
        let testdata = "A(2x2)BCD(2x2)EFG";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 11);
    }

    #[test]
    fn part1_example_5() {
        let testdata = "(6x1)(1x3)A";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 6);
    }

    #[test]
    fn part1_example_6() {
        let testdata = "X(8x2)(3x3)ABCY";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part1(), 18);
    }

    #[test]
    fn part2_example_1() {
        let testdata = "(3x3)XYZ";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 9);
    }

    #[test]
    fn part2_example_2() {
        let testdata = "X(8x2)(3x3)ABCY";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 20);
    }

    #[test]
    fn part2_example_3() {
        let testdata = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 241920);
    }

    #[test]
    fn part2_example_4() {
        let testdata = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let solution_data = InputData::try_from(testdata).unwrap();
        assert_eq!(solution_data.solve_part2(), 445);
    }
}
