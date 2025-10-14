// pandigital_products_simple.rs
// Project Euler 32 - Pandigital Products (simple, didactic version)
//
// We seek all products 'p = a * b' such that the concatenation "a b p"
// uses digits 1..9 exactly once each (no zeros), and then sum the *unique* products.
// Classic approach: iterate over the only viable digit-length splits (1x4 and 2x3),
// test the pandigital identity, and insert p into a HashSet to deduplicate.
//
// No spoilers: we print a placeholder rather than the final numeric sum.
// Run locally to reveal if you wish.

use std::collections::HashSet;

fn is_pandigital_identity(a: u32, b: u32) -> bool {
    let p = a * b;
    let s = format!("{}{}{}", a, b, p);
    if s.len() != 9 { return false; }             // must be exactly 9 digits total
    let mut seen = [false; 10];
    for ch in s.bytes() {
        if ch == b'0' { return false; }           // no zeros allowed
        let d = (ch - b'0') as usize;
        if seen[d] { return false; }              // each digit used once
        seen[d] = true;
    }
    // Ensure digits 1..9 all used
    (1..=9).all(|d| seen[d])
}

fn main() {
    let mut products = HashSet::<u32>::new();

    // Case 1: 1-digit * 4-digit = 4-digit
    for a in 1..=9 {
        for b in 1000..=9999 {
            if is_pandigital_identity(a, b) {
                products.insert(a * b);
            }
        }
    }

    // Case 2: 2-digit * 3-digit = 4-digit
    for a in 10..=99 {
        for b in 100..=999 {
            if is_pandigital_identity(a, b) {
                products.insert(a * b);
            }
        }
    }

    let sum: u32 = products.iter().sum();
    // println!("{}", sum); // TODO: reveal locally
    println!("sum of unique products = <redacted> ({} products)", products.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_identity() {
        // From the problem statement: 39 × 186 = 7254 forms a 1..9 pandigital identity.
        assert!(is_pandigital_identity(39, 186));
    }

    #[test]
    fn reject_zero_digit() {
        assert!(!is_pandigital_identity(10, 234)); // contains '0'
    }

    #[test]
    fn reject_wrong_length() {
        // 2 + 3 + 3 = 8 digits total; cannot be a valid 1..9 pandigital
        assert!(!is_pandigital_identity(12, 34));
    }
}
