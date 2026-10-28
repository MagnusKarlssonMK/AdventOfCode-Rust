//! # 2017 day 20 - Particle Swarm
//!
//! ## Part 1
//!
//! Note that the question is which particle will be closest *in the long term*, not the closest ever over the
//! entire trajectory. In the long term, the closest will be the one with the lowest acceleration. However, with my
//! input, there's multiple particles sharing the same lowest acceleration, and making a secondary sorting on initial
//! velocity is not safe since it depends on the relative direction between velocity and acceleration. Instead of
//! simulating step by step and hoping that a fixed number of steps is enough, we evaluate each particle's position
//! after `steps` steps in closed form:
//!
//! ### p(t) = p0 + v0 * t + a * t * (t + 1) / 2
//!
//! and pick the one with the smallest distance from the origin at that point in time.
//!
//! ## Part 2
//!
//! Simulate the swarm, updating particles in place, and after every step remove all particles that share a
//! position with another particle.
use crate::aoc_util::{
    error::*,
    point::{ORIGIN_3D, Point3D},
};
use std::{collections::HashMap, error::Error, str::FromStr};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2().to_string(),
    ))
}

const STEPS: i32 = 1000;

#[derive(Clone, Copy, Debug)]
struct Particle {
    point: Point3D,
    vel: Point3D,
    acc: Point3D,
}

impl FromStr for Particle {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(['<', '>']).collect::<Vec<_>>().as_slice() {
            ["p=", p, ", v=", v, ", a=", a, _] => Ok(Particle {
                point: p.parse().map_err(|_| AocError::Invalid(p.to_string()))?,
                vel: v.parse().map_err(|_| AocError::Invalid(v.to_string()))?,
                acc: a.parse().map_err(|_| AocError::Invalid(a.to_string()))?,
            }),
            _ => Err(AocError::Invalid(s.to_string())),
        }
    }
}

impl Particle {
    fn step(&mut self) {
        self.vel += self.acc;
        self.point += self.vel;
    }
}

#[derive(Debug)]
struct InputData {
    particles: Vec<Particle>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            particles: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl InputData {
    fn solve_part1(&self) -> usize {
        let triangular = STEPS * (STEPS + 1) / 2;
        let distances: Vec<_> = self
            .particles
            .iter()
            .map(|p| (p.point + p.vel * STEPS + p.acc * triangular).manhattan(&ORIGIN_3D))
            .collect();
        distances
            .iter()
            .enumerate()
            .min_by_key(|&(_, val)| val)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut particles = self.particles.to_vec();
        for _ in 0..STEPS {
            for p in particles.iter_mut() {
                p.step();
            }
            let mut counts = HashMap::new();
            for p in &particles {
                counts
                    .entry(p.point)
                    .and_modify(|v| *v += 1)
                    .or_insert(1usize);
            }
            particles = particles
                .iter()
                .filter(|p| counts[&p.point] == 1)
                .copied()
                .collect();
        }
        particles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

    /// Three particles moving in parallel, never sharing a position.
    const TEST_STRING_NO_COLLISIONS: &str = "p=<0,0,0>, v=<1,0,0>, a=<0,0,0>
p=<0,5,0>, v=<1,0,0>, a=<0,0,0>
p=<0,0,9>, v=<1,0,0>, a=<0,0,0>";

    /// Two particles that meet at the origin after the first step.
    const TEST_STRING_ALL_COLLIDE: &str = "p=<-1,0,0>, v=<1,0,0>, a=<0,0,0>
p=<1,0,0>, v=<-1,0,0>, a=<0,0,0>";

    /// One stationary particle is wiped out by another; a third far away survives.
    const TEST_STRING_PARTIAL_COLLIDE: &str = "p=<0,0,0>, v=<0,0,0>, a=<0,0,0>
p=<1,0,0>, v=<-1,0,0>, a=<0,0,0>
p=<2,0,0>, v=<0,0,0>, a=<0,0,0>";

    /// Non-zero acceleration: particle 1 stays put while particle 0 comes for it.
    const TEST_STRING_ACCELERATION: &str = "p=<0,0,0>, v=<0,0,0>, a=<1,0,0>
p=<10,0,0>, v=<0,0,0>, a=<0,0,0>";

    #[test]
    fn part1_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part1(), 3);
    }

    #[test]
    fn part2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part1_2_custom_1() {
        let solution_data = InputData::from_str(TEST_STRING_NO_COLLISIONS).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 3);
    }

    #[test]
    fn part1_2_custom_2() {
        let solution_data = InputData::from_str(TEST_STRING_ALL_COLLIDE).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 0);
    }

    #[test]
    fn part1_2_custom_3() {
        let solution_data = InputData::from_str(TEST_STRING_PARTIAL_COLLIDE).unwrap();
        assert_eq!(solution_data.solve_part1(), 0);
        assert_eq!(solution_data.solve_part2(), 1);
    }

    #[test]
    fn part1_2_custom_4() {
        let solution_data = InputData::from_str(TEST_STRING_ACCELERATION).unwrap();
        assert_eq!(solution_data.solve_part1(), 1);
        assert_eq!(solution_data.solve_part2(), 0);
    }
}
