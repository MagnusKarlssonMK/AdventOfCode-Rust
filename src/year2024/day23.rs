//! # 2024 day 23 - LAN Party
//!
//! Since the nodes in the input all consist of two lowercase characters, we can represent it with
//! a dense index in `0..676` (`26` * first + second`), and sets of nodes represented as fixed-size
//! bitsets. This means that in parsing it is made sure that each node is exactly two lowercase
//! letters.
//!
//! ## Part 1
//!
//! Iterate through combinations of three nodes, or triangles, where each triangle is visited once
//! as i < j < k, with k taken from the intersection of the neighbor bitsets. The triangle is added
//! to the count if any node falls in the `t` index range.
//!
//! ## Part 2
//!
//! This is pretty much a textbook example of the clique problem, so the Bron-Kerbosch algorithm
//! with pivoting is used to determine the answer.
use crate::aoc_util::error::AocError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<(String, String), AocError> {
    let solution_data: InputData = input.parse()?;
    Ok((
        solution_data.solve_part1().to_string(),
        solution_data.solve_part2(),
    ))
}

const LETTERS: usize = 26;
const BITSET_WIDTH: usize = 64;
const NODES: usize = LETTERS * LETTERS;
const WORDS: usize = NODES.div_ceil(BITSET_WIDTH);
/// Index range of all node names starting with 't'.
const T_RANGE: std::ops::Range<usize> = 19 * LETTERS..20 * LETTERS;

type BitSet = [u64; WORDS];

struct InputData {
    /// Adjacency bitset for each node.
    connections: Vec<BitSet>,
    /// All nodes present in the input
    nodes: BitSet,
}

fn parse_node(s: &str) -> Result<usize, AocError> {
    match s.as_bytes() {
        [a @ b'a'..=b'z', b @ b'a'..=b'z'] => {
            Ok(LETTERS * usize::from(a - b'a') + usize::from(b - b'a'))
        }
        _ => Err(AocError::Invalid(format!("computer name '{s}'"))),
    }
}

fn name_of(index: usize) -> String {
    let first = char::from(b'a' + (index / LETTERS) as u8);
    let second = char::from(b'a' + (index % LETTERS) as u8);
    format!("{first}{second}")
}

fn set_bit(set: &mut BitSet, index: usize) {
    set[index / BITSET_WIDTH] |= 1 << (index % BITSET_WIDTH);
}

fn clear_bit(set: &mut BitSet, index: usize) {
    set[index / BITSET_WIDTH] &= !(1 << (index % BITSET_WIDTH));
}

fn is_empty(set: &BitSet) -> bool {
    set.iter().all(|&w| w == 0)
}

fn count(set: &BitSet) -> u32 {
    set.iter().map(|w| w.count_ones()).sum()
}

fn and(a: &BitSet, b: &BitSet) -> BitSet {
    std::array::from_fn(|i| a[i] & b[i])
}

fn and_not(a: &BitSet, b: &BitSet) -> BitSet {
    std::array::from_fn(|i| a[i] & !b[i])
}

fn or(a: &BitSet, b: &BitSet) -> BitSet {
    std::array::from_fn(|i| a[i] | b[i])
}

/// Iterate over the indexes of all set bits in ascending order
fn iter_bits(set: &BitSet) -> impl Iterator<Item = usize> + '_ {
    set.iter().enumerate().flat_map(|(word_idx, &word)| {
        let mut remaining = word;
        std::iter::from_fn(move || {
            if remaining == 0 {
                None
            } else {
                let bit = remaining.trailing_zeros() as usize;
                remaining &= remaining - 1;
                Some(word_idx * BITSET_WIDTH + bit)
            }
        })
    })
}

impl FromStr for InputData {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections = vec![[0; WORDS]; NODES];
        let mut nodes = [0; WORDS];
        for line in s.lines() {
            let (left, right) = line
                .split_once('-')
                .ok_or_else(|| AocError::Invalid(format!("expected 'a-b': {line:?}")))?;
            let (a, b) = (parse_node(left)?, parse_node(right)?);
            if a == b {
                return Err(AocError::Invalid(format!("identical names: {line}")));
            }
            set_bit(&mut connections[a], b);
            set_bit(&mut connections[b], a);
            set_bit(&mut nodes, a);
            set_bit(&mut nodes, b);
        }
        Ok(Self { connections, nodes })
    }
}

impl InputData {
    /// Bron-Kerbosch with pivoting - finds all maximal cliques and keeps track of the largest.
    /// - `current` (R) - the clique being built
    /// - `candidates` (P) - vertices adjacent to everything in `current`, which may extend it.
    /// - `excluded` (X) - vertices adjacent to everything in `current` that have already been
    ///   handled.
    fn bron_kerbosch(
        &self,
        current: &mut Vec<usize>,
        mut candidates: BitSet,
        mut excluded: BitSet,
        largest_clique: &mut Vec<usize>,
    ) {
        if is_empty(&candidates) {
            // If also `excluded` is empty, `current` is a maximal clique.
            if is_empty(&excluded) && current.len() > largest_clique.len() {
                largest_clique.clone_from(current);
            }
            return;
        }

        // Pivot: the vertex in P ∪ X with most neighbors in P. Only candidates that are not
        // neighbors of the pivot need to be explored.
        let pivot = iter_bits(&or(&candidates, &excluded))
            .max_by_key(|&u| count(&and(&candidates, &self.connections[u])))
            .expect("candidates must be non-empty");
        let to_explore = and_not(&candidates, &self.connections[pivot]);

        for v in iter_bits(&to_explore) {
            current.push(v);
            self.bron_kerbosch(
                current,
                and(&candidates, &self.connections[v]),
                and(&excluded, &self.connections[v]),
                largest_clique,
            );
            current.pop();
            clear_bit(&mut candidates, v);
            set_bit(&mut excluded, v);
        }
    }

    fn solve_part1(&self) -> usize {
        let mut triangles = 0;
        for i in iter_bits(&self.nodes) {
            for j in iter_bits(&self.connections[i]).filter(|&j| j > i) {
                let common = and(&self.connections[i], &self.connections[j]);
                for k in iter_bits(&common).filter(|&k| k > j) {
                    if [i, j, k].iter().any(|n| T_RANGE.contains(n)) {
                        triangles += 1;
                    }
                }
            }
        }
        triangles
    }

    fn solve_part2(&self) -> String {
        let mut current = Vec::new();
        let mut largest_clique = Vec::new();
        self.bron_kerbosch(&mut current, self.nodes, [0; WORDS], &mut largest_clique);

        // Indexes are ordered like the names, so the result is already sorted.
        largest_clique.sort_unstable();
        largest_clique
            .into_iter()
            .map(name_of)
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_OFFICIAL_1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part1(), 7);
    }

    #[test]
    fn part2_official_1() {
        let solution_data: InputData = TEST_OFFICIAL_1.parse().unwrap();
        assert_eq!(solution_data.solve_part2(), "co,de,ka,ta");
    }

    #[test]
    fn parser_invalid_input() {
        for input in ["abcd", "ab-cde", "abc-de", "Aa-bc", "a1-bc", "aa-aa"] {
            assert!(matches!(
                input.parse::<InputData>(),
                Err(AocError::Invalid(_))
            ));
        }
    }
}
