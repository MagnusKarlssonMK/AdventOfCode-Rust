use std::collections::{HashSet, VecDeque};


use crate::aoc_util::point::*;

pub fn solve(input: &str) {
    let solution_data = InputData::parse_input(input);
    println!("Part 1: {}", solution_data.solve_part1());
    println!("Part 2: {}", solution_data.solve_part2());
}

struct Grid {
    x_max: usize,
    y_max: usize,
    elements: Vec<char>
}

impl Grid {
    #[inline]
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines()
            .map(|line| line.chars()
                .collect::<Vec<_>>())
            .collect();
        let x_max = lines[0].len();
        let y_max = lines.len();
        let mut elements = Vec::with_capacity(x_max * y_max);
        lines.iter().for_each(|line| line.iter().for_each(|c| elements.push(*c)));
        Self { x_max, y_max, elements }
    }

    #[inline]
    fn get_element(&self, p: &Point) -> Option<char> {
        if (0..self.x_max).contains(&(p.x as usize)) && 
                (0..self.y_max).contains(&(p.y as usize)) {
            Some(self.elements[self.x_max * (p.y as usize) + (p.x as usize)])
        } else {
            None
        }
    }
}

struct InputData {
    garden_map: Grid
}

impl InputData {
    fn parse_input(input: &str) -> Self {
        Self {
            garden_map: Grid::parse(input)
        }
    }

    fn solve_part1(&self) -> usize {
        let mut total = 0;
        let mut counted = HashSet:: new();
        for y in 0..self.garden_map.y_max {
            for x in 0..self.garden_map.x_max {
                if !counted.contains(&(x, y)) {
                    let mut area = 0;
                    let mut perimeter = 0;
                    let mut queue = VecDeque::from([Point::new(x as i32, y as i32)]);
                    while let Some(current) = queue.pop_front() {
                        if counted.contains(&(current.x as usize, current.y as usize)) {
                            continue;
                        }
                        counted.insert((current.x as usize, current.y as usize));
                        area += 1;
                        let element = self.garden_map.get_element(&current).unwrap();
                        for neighbor in NEIGHBORS_STRAIGHT.map(|dir| dir + current) {
                            if let Some(neighbor_val) = self.garden_map.get_element(&neighbor) {
                                if neighbor_val == element {
                                    queue.push_back(neighbor);
                                } else {
                                    perimeter += 1;
                                }
                            } else {
                                perimeter += 1;
                            }
                        }
                    }
                    total += area * perimeter;
                }
            }
        }
        total
    }

    fn solve_part2(&self) -> usize {
        let mut total = 0;
        let mut counted = HashSet:: new();
        for y in 0..self.garden_map.y_max {
            for x in 0..self.garden_map.x_max {
                if !counted.contains(&(x, y)) {
                    let mut area = 0;
                    let mut group = HashSet::new();
                    let mut queue = VecDeque::from([Point::new(x as i32, y as i32)]);
                    while let Some(current) = queue.pop_front() {
                        if counted.contains(&(current.x as usize, current.y as usize)) {
                            continue;
                        }
                        counted.insert((current.x as usize, current.y as usize));
                        group.insert(current);
                        area += 1;
                        let element = self.garden_map.get_element(&current).unwrap();
                        for neighbor in NEIGHBORS_STRAIGHT.map(|dir| dir + current) {
                            if let Some(neighbor_val) = self.garden_map.get_element(&neighbor) {
                                if neighbor_val == element {
                                    queue.push_back(neighbor);
                                } else {
                                    //perimeter += 1;
                                }
                            } else {
                                //perimeter += 1;
                            }
                        }
                    }
                    let mut sides_seen = HashSet::new();
                    for current in group.iter() {
                        for dir in &NEIGHBORS_STRAIGHT {
                            let neighbor = *dir + *current;
                            if group.contains(&neighbor) {
                                continue;
                            }
                            let mut feeler = *current;
                            while !group.contains(&(feeler + *dir)) && group.contains(&(feeler + Point::new(dir.y, dir.x))) {
                                feeler = Point::new(feeler.x + dir.y, feeler.y + dir.x);
                            }
                            if !sides_seen.contains(&(feeler, *dir)) {
                                sides_seen.insert((feeler, *dir));
                            }
                        }
                    }
                    total += area * sides_seen.len();
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let testdata = String::from(
"AAAA
BBCD
BBCC
EEEC");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 140);
    }

    #[test]
    fn part1_example_2() {
        let testdata = String::from(
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 772);
    }

    #[test]
    fn part1_example_3() {
        let testdata = String::from(
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part1(), 1930);
    }

    #[test]
    fn part2_example_1() {
        let testdata = String::from(
"AAAA
BBCD
BBCC
EEEC");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 80);
    }

    #[test]
    fn part2_example_2() {
        let testdata = String::from(
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 236);
    }

    #[test]
    fn part2_example_3() {
        let testdata = String::from(
"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA");
        let solution_data = InputData::parse_input(&testdata);
        assert_eq!(solution_data.solve_part2(), 368);
    }

}
