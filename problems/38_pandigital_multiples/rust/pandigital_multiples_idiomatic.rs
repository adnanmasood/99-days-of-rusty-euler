// pandigital_multiples_idiomatic.rs
// Project Euler 38 - Pandigital Multiples (idiomatic & slightly optimized)
//
// Observations used:
//  * The final number has 9 digits with digits 1..9 exactly once.
//  * If n = 2, the base x must be 4 digits and >= 5000 so that (x,2x) contributes 4+5 digits.
//  * If n = 3, the base x must be 3 digits and <= 3333 so that (x,2x,3x) contributes 3+3+3 digits.
//  * For n >= 4, even with 2-digit x we get at most 8 digits, so impossible.
//
// We scan the plausible ranges in descending order and stop early when we
// meet the first valid candidate (which maximizes the 9-digit value).
// A clean helper builds the concatenation and the bitmask-based pandigital test
// avoids allocations.

#[inline]
fn pow10(mut e: u32) -> u32 {
    let mut r = 1u32;
    while e > 0 { r *= 10; e -= 1; }
    r
}

#[inline]
fn digits(mut n: u32) -> u32 {
    if n == 0 { return 1; }
    let mut k = 0;
    while n > 0 { n /= 10; k += 1; }
    k
}

#[inline]
fn is_pandigital_1_to_9(mut n: u32) -> bool {
    let mut seen: u16 = 0;
    let mut count = 0;
    while n > 0 {
        let d = (n % 10) as u16;
        if d == 0 { return false; }
        let bit = 1u16 << d;
        if (seen & bit) != 0 { return false; }
        seen |= bit;
        count += 1;
        n /= 10;
    }
    count == 9 && seen == 0b1111111110
}

fn concat_until_9(x: u32) -> Option<(u32, u32)> {
    // returns (value, n) where value is the 9-digit concatenation, n is the last multiplier
    let mut value: u32 = 0;
    let mut len: u32 = 0;
    let mut k: u32 = 1;
    while len < 9 {
        let p = x * k;
        let dp = digits(p);
        value = value * pow10(dp) + p;
        len += dp;
        k += 1;
    }
    if len == 9 { Some((value, k - 1)) } else { None }
}

pub fn solve() -> u32 {
    // Try n = 2 candidates first: x in [9876..=5000].
    for x in (5000..=9876).rev() {
        if let Some((v, n)) = concat_until_9(x) {
            if n >= 2 && is_pandigital_1_to_9(v) {
                return v;
            }
        }
    }
    // Then n = 3 candidates: x in [3333..=100].
    for x in (100..=3333).rev() {
        if let Some((v, n)) = concat_until_9(x) {
            if n >= 2 && is_pandigital_1_to_9(v) {
                return v;
            }
        }
    }
    0
}

fn main() {
    let ans = solve();
    // println!("{}", ans); // TODO: reveal locally
    println!("largest pandigital concatenated product = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        // 9 -> 918273645
        let (v, n) = super::concat_until_9(9).unwrap();
        assert_eq!(n, 5);
        assert_eq!(v, 918273645);
        assert!(is_pandigital_1_to_9(v));
    }
}
