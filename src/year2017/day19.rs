//! # 2017 day 19 - A Series of Tubes
use crate::aoc_util::{grid::*, point::*};
use std::{error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input).unwrap();
    let (p1, p2) = solution_data.solve();
    Ok((p1, p2.to_string()))
}

struct InputData {
    grid: Grid,
}

impl FromStr for InputData {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::parse(s),
        })
    }
}

impl InputData {
    fn solve(&self) -> (String, usize) {
        let mut current = self.grid.find('|').unwrap();
        let mut direction = DOWN;
        let mut p1 = Vec::new();
        let mut p2 = 0;
        loop {
            p2 += 1;
            current += direction;
            let v = self.grid.get_element(&current).unwrap();
            match v {
                '+' => {
                    let left = current + direction.rotate_left();
                    if let Some(c) = self.grid.get_element(&left) {
                        if c != ' ' {
                            direction = direction.rotate_left();
                        } else {
                            direction = direction.rotate_right();
                        }
                    } else {
                        direction = direction.rotate_right();
                    }
                }
                ' ' => break,
                '-' | '|' => continue,
                _ => {
                    p1.push(v);
                }
            }
        }
        (p1.iter().collect(), p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";
        let solution_data = InputData::from_str(testdata).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, "ABCDEF");
        assert_eq!(p2, 38);
    }
}
