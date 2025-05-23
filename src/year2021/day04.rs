//! # 2021 day 4 - Giant Squid
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve_parts1_2();
    Ok((p1.to_string(), p2.to_string()))
}

struct Board {
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

impl FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        let nbr_rows = rows.len();
        let nbr_cols = rows[0].len();
        let cols: Vec<Vec<usize>> = (0..nbr_cols)
            .map(|col| (0..nbr_rows).map(|row| rows[row][col]).collect())
            .collect();
        Ok(Self { rows, cols })
    }
}

impl Board {
    fn get_bingoround_and_score(&self, drawmap: &HashMap<usize, usize>) -> (usize, usize) {
        let bingo_drawidx = self
            .rows
            .iter()
            .chain(self.cols.iter())
            .map(|row| row.iter().map(|v| drawmap.get(v).unwrap()).max().unwrap())
            .min()
            .unwrap();
        let mut score: usize = 0;
        for row in &self.rows {
            for nbr in row {
                if drawmap.get(nbr).unwrap() > bingo_drawidx {
                    score += nbr;
                }
            }
        }
        (*bingo_drawidx, score)
    }
}

struct InputData {
    draw: Vec<usize>,
    drawmap: HashMap<usize, usize>,
    boards: Vec<Board>,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let draw: Vec<usize> = blocks
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        let mut drawmap: HashMap<usize, usize> = HashMap::new();
        for (i, d) in draw.iter().enumerate() {
            drawmap.insert(*d, i);
        }
        let boards: Vec<Board> = blocks.map(|b| Board::from_str(b).unwrap()).collect();
        Ok(Self {
            draw,
            drawmap,
            boards,
        })
    }
}

impl InputData {
    fn solve_parts1_2(&self) -> (usize, usize) {
        let mut results: Vec<(usize, usize)> = self
            .boards
            .iter()
            .map(|board| board.get_bingoround_and_score(&self.drawmap))
            .collect();
        results.sort_unstable_by(|round, score| round.partial_cmp(score).unwrap());
        (
            self.draw[results.first().unwrap().0] * results.first().unwrap().1,
            self.draw[results.last().unwrap().0] * results.last().unwrap().1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_example_1() {
        let testdata = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve_parts1_2();
        assert_eq!(p1, 4512);
        assert_eq!(p2, 1924);
    }
}
