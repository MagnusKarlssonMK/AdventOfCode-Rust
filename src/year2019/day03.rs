use std::collections::HashSet;
use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    let (p1, p2) = solution_data.solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

impl Point {
    fn dir_to_point(d: char) -> Self {
        match d {
            'R' => RIGHT,
            'L' => LEFT,
            'U' => UP,
            'D' => DOWN,
            _ => unreachable!()
        }
    }
}

struct Wire {
    instructions: Vec<(Point, usize)>
}

impl Wire {
    fn parse_str(input: &str) -> Self {
        Self { instructions: input.split(',')
            .map(|s|
                (Point::dir_to_point(s.chars().nth(0).unwrap()),
                 s[1..].parse().unwrap()))
            .collect() }
    }

    fn walk(&self) -> Vec<Point> {
        let mut pos = ORIGIN;
        let mut points = Vec::new();
        for (p, v) in &self.instructions {
            for _ in 0..*v {
                pos += *p;
                if pos != ORIGIN {
                    points.push(pos);
                }
            }
        }
        points
    }
}

struct InputData {
    wire_1: Wire,
    wire_2: Wire,
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        let (left, right) = input.split_once('\n').unwrap();
        Self {
            wire_1: Wire::parse_str(left),
            wire_2: Wire::parse_str(right)
        }
    }

    fn solve(&self) -> (usize, usize) {
        let w1_points = self.wire_1.walk();
        let w2_points = self.wire_2.walk();
        let mut w1_set: HashSet<Point> = HashSet::new();
        w1_set.extend(&w1_points);
        let mut w2_set: HashSet<Point> = HashSet::new();
        w2_set.extend(&w2_points);
        let intersections = w1_set.intersection(&w2_set);
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        for i in intersections {
            p1.push(i.manhattan(&ORIGIN) as usize);
            p2.push(2 + w1_points.iter().position(|n| n == i).unwrap() + w2_points.iter().position(|n| n == i).unwrap());
        }
        (*p1.iter().min().unwrap(), *p2.iter().min().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from("R8,U5,L5,D3\nU7,R6,D4,L4");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, _) = solution_data.solve();
        assert_eq!(p1, 6);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 159);
        assert_eq!(p2, 610);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let solution_data = InputData::parse_input(&testdata);
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 135);
        assert_eq!(p2, 410);
    }

}
