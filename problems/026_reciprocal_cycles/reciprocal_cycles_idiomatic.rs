// reciprocal_cycles_idiomatic.rs
// Project Euler 26 — A more idiomatic and number-theoretic approach.
// We remove factors of 2 and 5, then compute the multiplicative order of 10 modulo d'.
// Final Euler answer intentionally not printed.

fn strip_twos_fives(mut n: u32) -> u32 {
    while n % 2 == 0 { n /= 2; }
    while n % 5 == 0 { n /= 5; }
    n
}

fn order_10(mut n: u32) -> u32 {
    // Assumes gcd(n, 10) = 1 (i.e., n has no factors 2 or 5). Returns the smallest k>0 with 10^k ≡ 1 (mod n).
    if n == 1 { return 0; } // terminating decimals have cycle 0
    let mut k = 1u32;
    let mut rem = 10 % n;
    while rem != 1 {
        rem = (rem * 10) % n;
        k += 1;
    }
    k
}

fn cycle_len(d: u32) -> u32 {
    let n = strip_twos_fives(d);
    order_10(n)
}

fn best_d_under(limit: u32) -> (u32, u32) {
    let mut best = (0u32, 0u32);
    for d in 2..limit {
        let len = cycle_len(d);
        if len > best.1 {
            best = (d, len);
        }
    }
    best
}

fn main() {
    // Sanity: 1/3 has cycle 1; 1/7 has cycle 6; 1/6 has cycle 1 (only the 3-part recurs)
    assert_eq!(cycle_len(3), 1);
    assert_eq!(cycle_len(7), 6);
    assert_eq!(cycle_len(6), 1);

    let (d, len) = best_d_under(1000);

    // println!("{} {}", d, len); // TODO: reveal locally
    println!("d<1000 with longest recurring cycle: d=[redacted], len=[redacted]");
    let _ = (d, len);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(cycle_len(2), 0);
        assert_eq!(cycle_len(3), 1);
        assert_eq!(cycle_len(7), 6);
        assert_eq!(cycle_len(8), 0);
        assert_eq!(cycle_len(9), 1);
        assert_eq!(cycle_len(10), 0);
    }
}
