// lattice_paths_idiomatic.rs
// Project Euler 15 — Lattice Paths (idiomatic Rust, no spoilers)

use std::cmp::min;

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Compute C(n, k) exactly in u128 using incremental reduction.
fn binom(n: u128, k: u128) -> u128 {
    let k = min(k, n - k);
    let mut res: u128 = 1;
    for i in 1..=k {
        let mut num = n - k + i; // term in numerator
        let mut den = i; // matching denominator term

        // reduce the current fraction first
        let g = gcd(num, den);
        num /= g;
        den /= g;

        // reduce denominator against accumulated result to keep numbers small
        let g2 = gcd(res, den);
        let res_div = res / g2;
        let den_div = den / g2;

        // multiply then divide (division is exact after reductions)
        res = res_div * num / den_div;
    }
    res
}

fn lattice_paths(n: u32) -> u128 {
    let n = n as u128;
    binom(2 * n, n)
}

fn main() {
    // Sanity checks
    assert_eq!(lattice_paths(0), 1);
    assert_eq!(lattice_paths(1), 2);
    assert_eq!(lattice_paths(2), 6);
    assert_eq!(lattice_paths(3), 20);

    // Compute for 20x20; keep redacted per no-spoiler policy.
    let ways = lattice_paths(20);
    let _ = ways;
    // println!("{}", ways); // TODO: reveal locally
    println!("Computed C(40,20) (redacted).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_grids() {
        assert_eq!(lattice_paths(2), 6);
        assert_eq!(lattice_paths(3), 20);
        assert_eq!(lattice_paths(4), 70);
    }
}
