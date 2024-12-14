use std::ops::{Add, AddAssign};

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

struct Hexpoint {
    q: isize,
    r: isize,
}

impl Hexpoint {
    fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }

    fn direction_from_str(input: &str) -> Self {
        match input {
            "n" => Self { q: 0, r: -1 },
            "ne" => Self { q: 1, r: -1 },
            "se" => Self { q: 1, r: 0 },
            "s" => Self { q: 0, r: 1 },
            "sw" => Self { q: -1, r: 1 },
            "nw" => Self { q: -1, r: 0 },
            _ => unreachable!(),
        }
    }

    fn get_distance(&self, other: &Self) -> usize {
        let s1 = -(self.q + self.r);
        let s2 = -(other.q + other.r);
        (((self.r - other.r).abs() + (self.q - other.q).abs() + (s1 - s2).abs()) / 2) as usize
    }
}

impl Add for Hexpoint {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.q + other.q, self.r + other.r)
    }
}

impl AddAssign for Hexpoint {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.q += other.q;
        self.r += other.r;
    }
}

struct InputData<'a> {
    moves: &'a str,
}

impl<'a> InputData<'a> {
    fn parse_input(input: &'a str) -> Self {
        Self { moves: input }
    }

    fn solve(&self) -> (usize, usize) {
        let start_point = Hexpoint::new(0, 0);
        let mut current_location = Hexpoint::new(0, 0);
        let mut current_distance = 0;
        let mut max_distance = 0;
        for m in self.moves.split(',') {
            current_location += Hexpoint::direction_from_str(m);
            current_distance = current_location.get_distance(&start_point);
            max_distance = max_distance.max(current_distance);
        }
        (current_distance, max_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("ne,ne,ne");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3);
        assert_eq!(p2, 3);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("ne,ne,sw,sw");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 0);
        assert_eq!(p2, 2);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("ne,ne,s,s");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 2);
        assert_eq!(p2, 2);
    }

    #[test]
    fn part1_example_4() {
        let testdata = String::from("se,sw,se,sw,sw");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 3);
        assert_eq!(p2, 3);
    }
}
