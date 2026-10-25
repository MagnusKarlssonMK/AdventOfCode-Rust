//! # 2016 day 24 - Air Duct Spelunking
//!
//! Travelling Salesman problem.
//!
//! Takes the input grid, finds the digits and runs a BFS for each one to map out the distances
//! between them all, and store it in a distance matrix.
//!
//! Then to solve the problem, Held-Karp algorithm is used to find the shortest paths for
//! both part 1 and part 2 in one go.
use crate::aoc_util::{error::*, grid::Grid, point::*};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution_data = InputData::from_str(input)?;
    let (p1, p2) = solution_data.solve();
    Ok((p1.to_string(), p2.to_string()))
}

#[derive(Debug)]
struct InputData {
    index_map: Vec<char>,
    distance_matrix: Vec<Vec<usize>>,
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn add_edge(
            graph: &mut HashMap<char, HashMap<char, usize>>,
            a: char,
            b: char,
            steps: usize,
        ) {
            graph.entry(a).or_default().entry(b).or_insert(steps);
            graph.entry(b).or_default().entry(a).or_insert(steps);
        }

        let grid = Grid::parse(s);
        let digits: HashMap<_, _> = grid
            .elements
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_digit())
            .map(|(i, c)| (grid.get_point(i), *c))
            .collect();
        let nbr_of_digits = digits.keys().len();

        // Build adjacency map with BFS from the found digits
        let mut graph: HashMap<char, HashMap<char, usize>> = HashMap::new();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        // Paths go both ways, i.e. we can skip one starting point and still find all connections to it
        for (start_point, start_digit) in digits.iter() {
            queue.clear();
            queue.push_back((*start_point, 0));
            seen.clear();
            seen.insert(*start_point);
            while let Some((point, steps)) = queue.pop_front() {
                if let Some(current_digit) = digits.get(&point)
                    && current_digit != start_digit
                {
                    // Add path to adj in both directions.
                    // Paths may be found multiple times, but this will only write the first time since that will be the shortest one.
                    add_edge(&mut graph, *start_digit, *current_digit, steps);
                }

                if graph
                    .get(start_digit)
                    .is_some_and(|v| v.keys().len() == nbr_of_digits - 1)
                {
                    // All paths to other nodes have been found for this node; we can stop
                    break;
                }

                for n in NEIGHBORS_STRAIGHT.iter().map(|d| *d + point) {
                    if let Some(v) = grid.get_element(&n)
                        && v != '#'
                        && !seen.contains(&n)
                    {
                        seen.insert(n);
                        queue.push_back((n, steps + 1));
                    }
                }
            }
            if graph
                .get(start_digit)
                .is_none_or(|v| v.keys().len() < nbr_of_digits - 1)
            {
                // The input is unsolvable if any digits are walled off
                return Err(AocError::Invalid(format!("missing path(s): {start_digit}")));
            }
        }

        let mut index_map: Vec<_> = digits.values().copied().collect();
        index_map.sort_unstable();
        if *index_map.first().ctx_invalid("no nodes")? != '0' {
            // is_digit() further up should guarantee that '0' is first after the sort, since negative values are impossible with a single character
            return Err(AocError::Invalid("node zero not found".to_string()));
        }
        let mut distance_matrix = vec![vec![0usize; nbr_of_digits]; nbr_of_digits];

        for (i, c) in index_map.iter().enumerate() {
            for (j, d) in index_map.iter().enumerate() {
                if let Some(x) = graph[c].get(d) {
                    distance_matrix[i][j] = *x;
                }
            }
        }

        Ok(Self {
            index_map,
            distance_matrix,
        })
    }
}

impl InputData {
    fn solve(&self) -> (usize, usize) {
        // Solve both parts using Held-Karp
        let n = self.index_map.len();
        let mut dp = vec![vec![usize::MAX / 4; n]; 1 << n];
        dp[1][0] = 0;

        for mask in 0..(1 << n) {
            for i in 0..n {
                if (mask & (1 << i)) == 0 {
                    continue;
                }
                let cur = dp[mask][i];
                if cur >= usize::MAX / 4 {
                    continue;
                }
                #[allow(clippy::needless_range_loop)]
                for j in 0..n {
                    if (mask & (1 << j)) != 0 {
                        continue;
                    }
                    let nd = cur + self.distance_matrix[i][j];
                    let nmask = mask | (1 << j);
                    if nd < dp[nmask][j] {
                        dp[nmask][j] = nd;
                    }
                }
            }
        }

        let full = (1 << n) - 1;
        // Part 1: visit all, end anywhere
        let p1 = (0..n).map(|i| dp[full][i]).min().unwrap();

        // Part 2: visit all, return to start
        let p2 = (0..n)
            .map(|i| dp[full][i] + self.distance_matrix[i][0])
            .min()
            .unwrap();

        (p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    const TEST_CUSTOM_ISOLATED: &str = "#########
#0.1#2.3#
#########";

    #[test]
    fn part1_2_official_1() {
        let solution_data = InputData::from_str(TEST_OFFICIAL_1).unwrap();
        let (p1, p2) = solution_data.solve();
        assert_eq!(p1, 14);
        assert_eq!(p2, 20);
    }

    #[test]
    fn parser_isolated_nodes() {
        let err = InputData::from_str(TEST_CUSTOM_ISOLATED).unwrap_err();
        assert!(matches!(err, AocError::Invalid(_)));
    }
}
