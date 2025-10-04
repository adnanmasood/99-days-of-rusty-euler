// longest_collatz_sequence_idiomatic.rs
// Project Euler 14 - Longest Collatz Sequence
// Faster approach: memoization for chain lengths; cache for small values and HashMap for large.

use std::collections::HashMap;

#[inline]
fn next_collatz(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

// Returns the number of terms from n down to 1 (inclusive).
// Uses dynamic programming with a small vector cache and a HashMap for overflow.
fn collatz_len(n: u64, small: &mut [u32], big: &mut HashMap<u64, u32>) -> u32 {
    if n == 1 { return 1; }

    let mut x = n;
    let mut trail: Vec<u64> = Vec::new();
    let mut known: u32;

    loop {
        if x == 1 {
            known = 1;
            break;
        }
        if (x as usize) < small.len() {
            let v = small[x as usize];
            if v != 0 { known = v; break; }
        }
        if let Some(&v) = big.get(&x) {
            known = v; break;
        }
        trail.push(x);
        x = next_collatz(x);
    }

    // Reconstruct lengths backwards and store them for reuse.
    let mut len = known;
    for &y in trail.iter().rev() {
        len += 1;
        if (y as usize) < small.len() {
            small[y as usize] = len;
        } else {
            big.insert(y, len);
        }
    }
    len
}

fn main() {
    const LIMIT: usize = 1_000_000;            // starting numbers are 1..LIMIT-1
    let mut small: Vec<u32> = vec![0; LIMIT + 1];
    small[1] = 1; // base case
    let mut big: HashMap<u64, u32> = HashMap::new();

    let mut best_start: u64 = 1;
    let mut best_len: u32 = 1;

    for start in 1..LIMIT as u64 {
        let l = collatz_len(start, &mut small, &mut big);
        if l > best_len {
            best_len = l;
            best_start = start;
        }
    }

    // println!("Best start < {} is {} with length {}", LIMIT, best_start, best_len); // TODO
    println!("Computed longest chain under 1e6 (idiomatic). (Redacted.)");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collatz_13_has_10_terms() {
        let mut small = vec![0u32; 1000];
        small[1] = 1;
        let mut big = HashMap::new();
        let len = collatz_len(13, &mut small, &mut big);
        assert_eq!(len, 10); // 13 -> ... -> 1 has 10 terms
    }
}
