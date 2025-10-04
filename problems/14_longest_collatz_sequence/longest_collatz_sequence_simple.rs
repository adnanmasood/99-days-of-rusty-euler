// longest_collatz_sequence_simple.rs
// Project Euler 14 - Longest Collatz Sequence
// Naive scan without memoization: compute the chain length for each start.
// We intentionally avoid printing the final numeric answer (no-spoiler policy).

fn collatz_len(mut n: u128) -> u32 {
    // Definition: length counts *terms*, so L(1) = 1, L(2) = 2 (2 -> 1), etc.
    let mut len: u32 = 1;
    while n != 1 {
        if n % 2 == 0 { n /= 2; } else { n = 3 * n + 1; }
        len += 1;
    }
    len
}

fn main() {
    let limit: u128 = 1_000_000; // "under one million" => 1..999_999
    let mut best_start: u128 = 1;
    let mut best_len: u32 = 1;

    for start in 1..limit {
        let l = collatz_len(start);
        if l > best_len {
            best_len = l;
            best_start = start;
        }
    }

    // println!("Best start < 1_000_000 is {} with length {}", best_start, best_len); // TODO
    println!("Computed longest chain under 1e6 (redacted).");
}
