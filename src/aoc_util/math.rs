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

/// Calculates the Least Common Multiple for a slice of integers.
///
/// # Panic
///
/// Will panic if the slice is empty.
pub fn vec_lcm(input: &[usize]) -> usize {
    assert!(!input.is_empty(), "vec_lcm requires a non-empty slice");
    input.iter().copied().reduce(lcm).unwrap()
}

/// Extended Euclidean Algorithm: returns `(g, x, y)` such that
/// `a * x + b * y == g`, where `g == gcd(a, b)` is always non-negative.
pub fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b.abs(), 0, b.signum())
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

/// Computes a^(-1) mod m
pub fn mod_inverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // Inverse doesn't exist if they aren't coprime
    } else {
        Some((x % m + m) % m)
    }
}

/// Chinese Remainder Theorem calculation
pub fn chinese_remainder(moduli: &[i128], remainders: &[i128]) -> Option<i128> {
    if moduli.is_empty() || moduli.len() != remainders.len() || moduli.iter().any(|n| *n <= 0) {
        return None;
    }

    // Calculate the product of all moduli
    let mut prod: i128 = 1;
    for &n in moduli {
        prod = prod.checked_mul(n)?;
    }

    let mut result: i128 = 0;
    for (&n, &r) in moduli.iter().zip(remainders.iter()) {
        let partial = prod / n;
        let inv = mod_inverse(partial, n)?;

        let term = r.checked_mul(inv)?.checked_mul(partial)?;
        result = result.checked_add(term)?;
    }

    Some((result % prod + prod) % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_successful() {
        assert_eq!(chinese_remainder(&[3, 5, 7], &[2, 3, 2]).unwrap(), 23);
        assert_eq!(chinese_remainder(&[5, 11], &[0, 9]).unwrap(), 20);
        assert_eq!(chinese_remainder(&[4], &[3]).unwrap(), 3);
    }

    #[test]
    fn test_crt_no_solution() {
        assert_eq!(chinese_remainder(&[0, 5, 7], &[2, 3, 2]), None);
        assert_eq!(chinese_remainder(&[3, 5, 7], &[2, 3]), None);
        assert_eq!(chinese_remainder(&[2, 4, 7], &[2, 3, 2]), None);
    }

    #[test]
    fn test_ext_gcd() {
        assert_eq!(extended_gcd(30, 20), (10, 1, -1));
        assert_eq!(extended_gcd(3, 5), (1, 2, -1));
        assert_eq!(extended_gcd(0, 5), (5, 0, 1));
        // Negative inputs must still yield a non-negative gcd.
        assert_eq!(extended_gcd(3, -5), (1, 2, 1));
        assert_eq!(extended_gcd(-3, 5), (1, -2, -1));
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 11), Some(4));
        assert_eq!(mod_inverse(-3, 5), Some(3));
        assert_eq!(mod_inverse(2, 4), None);
    }

    #[test]
    fn test_vec_lcm() {
        assert_eq!(vec_lcm(&[7]), 7);
        assert_eq!(vec_lcm(&[4, 6]), 12);
        assert_eq!(vec_lcm(&[2, 3, 5]), 30);
    }
}
