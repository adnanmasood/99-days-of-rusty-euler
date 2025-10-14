// pandigital_multiples_simple.rs
// Project Euler 38 - Pandigital Multiples (simple, didactic version)
//
// Find the largest 1..9 pandigital 9-digit number formed as the concatenated
// product of an integer with (1,2,...,n) where n > 1.
//
// Approach:
//  - For each x in 1..=9999 (x must be <= 4 digits; otherwise x*1 already 5+ digits),
//    build the concatenated product x * 1 || x * 2 || ... until its length >= 9.
//  - If the length is exactly 9 and we used at least two factors (n>=2), test if
//    the 9 digits are a permutation of 1..9 (no zeros, no repeats). Keep the max.
//
// This version favors clarity over micro-optimizations.
//
// No spoilers: we print a placeholder rather than the final value.

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
    // Exactly nine digits, each 1..9 exactly once.
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
    count == 9 && seen == 0b1111111110  // bits 1..9 set
}

fn concatenated_product_value(x: u32) -> Option<u32> {
    // Build x*1 || x*2 || ... until length >= 9.
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
    if len == 9 && (k - 1) >= 2 { Some(value) } else { None }
}

fn main() {
    let mut best: u32 = 0;
    for x in 1..=9999 {
        if let Some(v) = concatenated_product_value(x) {
            if is_pandigital_1_to_9(v) && v > best {
                best = v;
            }
        }
    }
    // println!("{}", best); // TODO: reveal locally
    println!("largest pandigital concatenated product = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_9() {
        // From the statement: 9 concatenated with (1..5) is 918273645.
        // Our function builds until 9 digits automatically.
        let v = concatenated_product_value(9).unwrap();
        assert_eq!(v, 918273645);
        assert!(is_pandigital_1_to_9(v));
    }
}
