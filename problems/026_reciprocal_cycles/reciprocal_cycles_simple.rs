// reciprocal_cycles_simple.rs
// Project Euler 26 — Reciprocal Cycles (simple long-division simulation).
// Final Euler answer intentionally not printed for the 99‑day series.

use std::cmp::max;

fn cycle_len_of_unit_fraction(d: usize) -> usize {
    // Long-division style: track first occurrence of each remainder.
    // 1/d -> start with remainder 1 % d; multiply by 10 each step.
    let mut first_seen = vec![None; d];
    let mut rem = 1 % d;
    let mut pos = 0usize;
    while rem != 0 && first_seen[rem].is_none() {
        first_seen[rem] = Some(pos);
        rem = (rem * 10) % d;
        pos += 1;
    }
    match rem {
        0 => 0, // terminating decimal; cycle length 0
        _ => pos - first_seen[rem].unwrap(),
    }
}

fn best_d_under(limit: usize) -> (usize, usize) {
    let mut best_d = 0usize;
    let mut best_len = 0usize;
    for d in 2..limit {
        let len = cycle_len_of_unit_fraction(d);
        if len > best_len {
            best_len = len;
            best_d = d;
        }
    }
    (best_d, best_len)
}

fn main() {
    // Sanity checks from the statement:
    assert_eq!(cycle_len_of_unit_fraction(2), 0); // 1/2 = 0.5 (terminates)
    assert_eq!(cycle_len_of_unit_fraction(3), 1); // 0.(3)
    assert_eq!(cycle_len_of_unit_fraction(7), 6); // 0.(142857)

    let (d, len) = best_d_under(1000);

    // println!("{} {}", d, len); // TODO: reveal locally
    println!("d<1000 with longest recurring cycle: d=[redacted], len=[redacted]");

    // Avoid unused warning
    let _ = (d, len);
}
