//! # Math related utility functions

/// Calculates the Greatest Common Divisor for two integers.
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Calculates the Least Common Multiple for two integers.
pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

/// Calculates the Least Common Multiple for an array of integers.
///
/// # Panic
///
/// Will panic if the length of the input is zero.
pub fn vec_lcm(input: &[usize]) -> usize {
    assert!(!input.is_empty());
    let mut nbrs = input.to_vec();

    loop {
        let mut done = true;
        for n in nbrs.iter().skip(1) {
            if n != nbrs.first().unwrap() {
                done = false;
                break;
            }
        }
        if done {
            return *nbrs.first().unwrap();
        }

        let lowest_idx = nbrs
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        nbrs[lowest_idx] += input[lowest_idx];
    }
}
