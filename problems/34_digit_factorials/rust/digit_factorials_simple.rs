// digit_factorials_simple.rs
// Project Euler 34 - Digit Factorials (simple, didactic version)
//
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
// Exclude 1 and 2 (they are not sums).
// We'll precompute factorials for digits 0..9 and scan up to a safe upper bound: 7 * 9!
// (Because for k >= 8, k*9! < 10^(k-1), so no k-digit number can match.)
//
// No spoilers: we print a placeholder instead of the final sum. Reveal locally if you wish.

const FACT: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
const UPPER: u32 = 7 * 362880; // 2_540_160

fn digit_factorial_sum(mut n: u32) -> u32 {
    let mut s = 0u32;
    if n == 0 { return FACT[0]; }
    while n > 0 {
        let d = (n % 10) as usize;
        s += FACT[d];
        n /= 10;
    }
    s
}

fn main() {
    let mut total: u64 = 0;
    for n in 10..=UPPER {
        if n == digit_factorial_sum(n) {
            total += n as u64;
        }
    }
    // println!("{}", total); // TODO: reveal locally
    println!("sum of curious numbers = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_lookup() {
        assert_eq!(FACT[0], 1);
        assert_eq!(FACT[5], 120);
        assert_eq!(FACT[9], 362880);
    }

    #[test]
    fn example_145_matches() {
        assert_eq!(digit_factorial_sum(145), 145);
    }
}
