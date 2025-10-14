// truncatable_primes_idiomatic.rs
// Project Euler 37 - Truncatable Primes (idiomatic & robust Rust)
//
// Same idea as the simple version, but we keep helpers tiny, use sets for
// de-duplication and a single `solve()` entry point suitable for tests.

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
fn pow10(exp: u32) -> u64 {
    let mut r = 1u64;
    for _ in 0..exp { r *= 10; }
    r
}

fn grow_right_truncatable(layer: &[u64]) -> Vec<u64> {
    let mut next = Vec::new();
    for &p in layer {
        for &d in &[1u64,3,7,9] {
            let n = p * 10 + d;
            if is_prime(n) { next.push(n); }
        }
    }
    next
}

fn grow_left_truncatable(layer: &[u64]) -> Vec<u64> {
    let mut next = Vec::new();
    for &p in layer {
        let base = pow10(digits_count(p));
        for &d in &[1u64,2,3,5,7,9] {
            let n = d * base + p;
            if is_prime(n) { next.push(n); }
        }
    }
    next
}

fn build_truncatable<F>(mut grow: F) -> HashSet<u64>
where F: FnMut(&[u64]) -> Vec<u64> {
    let mut result = HashSet::new();
    let mut layer = vec![2u64,3,5,7];
    loop {
        let next = grow(&layer);
        for &n in &next { if n >= 10 { result.insert(n); } }
        if next.is_empty() { break; }
        layer = next;
    }
    result
}

pub fn solve() -> u64 {
    let right = build_truncatable(grow_right_truncatable);
    let left  = build_truncatable(grow_left_truncatable);

    left.intersection(&right)
        .filter(|&&n| n >= 10)
        .map(|&n| n)
        .filter(|&n| {
            // double-check by truncating both directions
            let mut x = n;
            while x > 0 { if !is_prime(x) { return false; } x /= 10; }
            let mut base = pow10(digits_count(n)-1);
            let mut y = n;
            while base > 0 { if !is_prime(y) { return false; } y %= base; base /= 10; }
            true
        })
        .map(|n| n as u64)
        .sum::<u64>()
}

fn main() {
    let ans = solve();
    // println!("{}", ans); // TODO: reveal locally
    println!("sum of the eleven truncatable primes = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        // 3797 is truncatable both ways per statement.
        let mut x = 3797u64;
        while x > 0 { assert!(is_prime(x)); x /= 10; }
        let mut base = pow10(digits_count(3797)-1);
        let mut y = 3797u64;
        while base > 0 { assert!(is_prime(y)); y %= base; base /= 10; }
    }
}
