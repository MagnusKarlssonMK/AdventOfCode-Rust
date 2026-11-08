//! # Math related utility functions
use std::ops::Rem;

/// Types that support a greatest-common-divisor computation.
pub trait Gcd: Copy + PartialEq + From<u8> + Rem<Output = Self> {
    fn gcd(self, other: Self) -> Self {
        let zero = Self::from(0);
        let (mut a, mut b) = (self, other);
        while b != zero {
            (a, b) = (b, a % b);
        }
        a
    }
}

impl Gcd for usize {}
impl Gcd for u128 {}

/// Calculates the Greatest Common Divisor for two integers.
pub fn gcd<T: Gcd>(a: T, b: T) -> T {
    Gcd::gcd(a, b)
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

/// Errors returned by [`chinese_remainder`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrtError {
    /// No moduli were provided.
    Empty,
    /// The number of moduli and remainders differs.
    LengthMismatch,
    /// A modulus was zero or negative.
    NonPositiveModulus,
    /// The congruences have no common solution.
    Inconsistent,
    /// A value or intermediate result did not fit in `i128`.
    Overflow,
}

/// Solves `x ≡ remainders[i] (mod moduli[i])` using the generalized CRT.
///
/// Moduli do not need to be pairwise coprime. Returns the smallest
/// non-negative solution. Values that don't fit in `i128`, or intermediate
/// arithmetic that overflows, yield `CrtError::Overflow`.
pub fn chinese_remainder<M, R>(moduli: &[M], remainders: &[R]) -> Result<i128, CrtError>
where
    M: Copy + TryInto<i128>,
    R: Copy + TryInto<i128>,
{
    if moduli.is_empty() {
        return Err(CrtError::Empty);
    }
    if moduli.len() != remainders.len() {
        return Err(CrtError::LengthMismatch);
    }

    let mut result: i128 = 0;
    let mut product: i128 = 1;

    for (&modulus, &remainder) in moduli.iter().zip(remainders) {
        let n: i128 = modulus.try_into().map_err(|_| CrtError::Overflow)?;
        let r: i128 = remainder.try_into().map_err(|_| CrtError::Overflow)?;
        if n <= 0 {
            return Err(CrtError::NonPositiveModulus);
        }
        let r = r.rem_euclid(n);

        // product * x + n * y == g, so x is the inverse of product / g mod n / g.
        let (g, x, _) = extended_gcd(product, n);
        let diff = (r - result).rem_euclid(n);
        if diff % g != 0 {
            return Err(CrtError::Inconsistent);
        }

        let n_g = n / g;
        let inv = x.rem_euclid(n_g);
        let k = (diff / g)
            .checked_mul(inv)
            .ok_or(CrtError::Overflow)?
            .rem_euclid(n_g);
        let new_product = (product / g).checked_mul(n).ok_or(CrtError::Overflow)?;
        result = result
            .checked_add(product.checked_mul(k).ok_or(CrtError::Overflow)?)
            .ok_or(CrtError::Overflow)?
            .rem_euclid(new_product);
        product = new_product;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_successful() {
        assert_eq!(chinese_remainder(&[3, 5, 7], &[2, 3, 2]), Ok(23));
        assert_eq!(chinese_remainder(&[5, 11], &[0, 9]), Ok(20));
        assert_eq!(chinese_remainder(&[4], &[3]), Ok(3));
        assert_eq!(chinese_remainder(&[6, 10], &[0, 2]), Ok(12)); // non-coprime, consistent
        assert_eq!(chinese_remainder(&[6, 10], &[0, 8]), Ok(18)); // non-coprime, consistent
        assert_eq!(chinese_remainder(&[5, 7], &[-1, -2]), Ok(19)); // negative remainders
        assert_eq!(chinese_remainder(&[1, 1], &[5, 7]), Ok(0));
        assert_eq!(chinese_remainder(&[6u128, 10], &[0u128, 2]), Ok(12)); // generic input
    }

    #[test]
    fn test_crt_errors() {
        assert_eq!(
            chinese_remainder::<i128, i128>(&[], &[]),
            Err(CrtError::Empty)
        );
        assert_eq!(
            chinese_remainder(&[3, 5, 7], &[2, 3]),
            Err(CrtError::LengthMismatch)
        );
        assert_eq!(
            chinese_remainder(&[0, 5, 7], &[2, 3, 2]),
            Err(CrtError::NonPositiveModulus)
        );
        assert_eq!(
            chinese_remainder(&[2, 4, 7], &[2, 3, 2]),
            Err(CrtError::Inconsistent)
        );
        assert_eq!(
            chinese_remainder(&[6, 3], &[0, 2]),
            Err(CrtError::Inconsistent)
        );
        assert_eq!(
            chinese_remainder(&[6, 10], &[0, 9]),
            Err(CrtError::Inconsistent)
        );
        assert_eq!(
            chinese_remainder(&[i128::MAX, i128::MAX - 1], &[0, 0]),
            Err(CrtError::Overflow)
        );
        assert_eq!(
            chinese_remainder(&[u128::MAX], &[0u128]),
            Err(CrtError::Overflow)
        );
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
