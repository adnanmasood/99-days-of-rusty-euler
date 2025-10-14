// truncatable_primes_simple.rs
// Project Euler 37 - Truncatable Primes (simple, didactic version)
//
// We find the (exactly eleven) primes that remain prime when truncating
// digits from left to right and from right to left. We then sum them.
// The problem's note: 2, 3, 5, 7 are *not* considered truncatable (single digit).
//
// Strategy (clear over clever):
// 1) Build the set of right-truncatable primes by BFS: start with {2,3,5,7},
//    and append digits {1,3,7,9}. If the new number is prime, keep it.
// 2) Build the set of left-truncatable primes by BFS: start with {2,3,5,7},
//    and *prefix* digits {1,2,3,5,7,9}. If the new number is prime, keep it.
// 3) Intersect the two sets and sum numbers >= 10.
//
// To keep the code self-contained, primality is tested by trial division up to sqrt(n).
// This is fast enough because the search trees are tiny.
//
// No spoilers: we print a placeholder instead of the final sum. Reveal locally if you wish.

use std::collections::HashSet;

#[inline]
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n % 2 == 0 { return n == 2; }
    if n % 3 == 0 { return n == 3; }
    let mut f = 5u64;
    while f * f <= n {
        if n % f == 0 || n % (f + 2) == 0 { return false; }
        f += 6;
    }
    true
}

#[inline]
fn digits_count(mut n: u64) -> u32 {
    if n == 0 { return 1; }
    let mut k = 0;
    while n > 0 { n /= 10; k += 1; }
    k
}

#[inline]
fn pow10(mut e: u32) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        r *= 10;
        e -= 1;
    }
    r
}

fn build_right_truncatable() -> HashSet<u64> {
    let mut result = HashSet::new();
    let mut layer = vec![2u64, 3, 5, 7];
    let add = [1u64, 3, 7, 9];
    loop {
        let mut next = Vec::new();
        for &p in &layer {
            if p >= 10 { result.insert(p); }
            for &d in &add {
                let n = p * 10 + d;
                if is_prime(n) {
                    next.push(n);
                }
            }
        }
        if next.is_empty() { break; }
        layer = next;
    }
    result
}

fn build_left_truncatable() -> HashSet<u64> {
    let mut result = HashSet::new();
    let mut layer = vec![2u64, 3, 5, 7];
    let add = [1u64, 2, 3, 5, 7, 9];
    loop {
        let mut next = Vec::new();
        for &p in &layer {
            if p >= 10 { result.insert(p); }
            let k = digits_count(p);
            let base = pow10(k);
            for &d in &add {
                let n = d * base + p;
                if is_prime(n) {
                    next.push(n);
                }
            }
        }
        if next.is_empty() { break; }
        layer = next;
    }
    result
}

fn is_truncatable_both(n: u64) -> bool {
    if n < 10 || !is_prime(n) { return false; }
    // right to left
    let mut x = n;
    while x > 0 {
        if !is_prime(x) { return false; }
        x /= 10;
    }
    // left to right
    let mut base = pow10(digits_count(n) - 1);
    let mut y = n;
    while base > 0 {
        if !is_prime(y) { return false; }
        y %= base;
        base /= 10;
    }
    true
}

fn main() {
    let right = build_right_truncatable();
    let left = build_left_truncatable();

    let mut total: u64 = 0;
    let mut count = 0usize;
    for &n in right.intersection(&left) {
        if n >= 10 {
            // Optional guard; also verify by direct predicate
            if is_truncatable_both(n) {
                total += n;
                count += 1;
            }
        }
    }
    println!("found {} truncatable primes (should be 11)", count);
    // println!("{}", total); // TODO: reveal locally
    println!("sum of the eleven truncatable primes = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_3797() {
        assert!(is_truncatable_both(3797));
    }
}
