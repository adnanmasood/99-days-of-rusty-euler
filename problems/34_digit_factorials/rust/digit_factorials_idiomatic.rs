// digit_factorials_idiomatic.rs
// Project Euler 34 - Digit Factorials (idiomatic & robust Rust)
//
// This version uses iterator-style digit processing, const tables for factorials,
// and tight numeric types. The algorithm is the same as the simple version.

const FACT: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
const UPPER: u32 = 7 * 362880; // Proof-by-bound: 8*9! < 10^7, so 7*9! is a safe top

#[inline]
fn digit_factorial_sum(mut n: u32) -> u32 {
    if n == 0 { return FACT[0]; }
    let mut acc = 0u32;
    while n > 0 {
        acc += FACT[(n % 10) as usize];
        n /= 10;
    }
    acc
}

pub fn solve() -> u64 {
    (10..=UPPER)
        .filter(|&n| n == digit_factorial_sum(n))
        .map(|n| n as u64)
        .sum::<u64>()
}

fn main() {
    let ans = solve();
    // println!("{}", ans); // TODO: reveal locally
    println!("sum of curious numbers = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(digit_factorial_sum(145), 145);
    }

    #[test]
    fn bounds_reasonable() {
        // Sanity: UPPER should be 2_540_160
        assert_eq!(UPPER, 2_540_160);
    }
}
