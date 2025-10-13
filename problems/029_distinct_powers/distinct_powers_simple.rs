// distinct_powers_simple.rs
// Project Euler 29 — Distinct Powers (simple canonicalization by prime exponents).
// We avoid big integers by representing a^b via its prime-exponent vector.
// Final Euler answer intentionally not printed for the 99-day series.

use std::collections::HashSet;

fn sieve_primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2usize;
    while p * p <= limit {
        if is_prime[p] {
            let mut m = p * p;
            while m <= limit {
                is_prime[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    (2..=limit).filter(|&x| is_prime[x]).collect()
}

fn factor_exponents(mut n: usize, primes: &[usize]) -> Vec<u16> {
    let mut exps = vec![0u16; primes.len()];
    for (i, &p) in primes.iter().enumerate() {
        if p * p > n { break; }
        while n % p == 0 {
            exps[i] += 1;
            n /= p;
        }
    }
    if n > 1 {
        // n is prime and <= 100, so it must be in `primes` near the end
        if let Some(pos) = primes.iter().position(|&q| q == n) {
            exps[pos] += 1;
        } else {
            // Should not happen for n <= 100, but keep safe.
            panic!("Prime {} not in list", n);
        }
    }
    exps
}

fn count_distinct(a_lo: usize, a_hi: usize, b_lo: usize, b_hi: usize) -> usize {
    let primes = sieve_primes(a_hi);
    let mut seen: HashSet<Vec<u16>> = HashSet::new();

    for a in a_lo..=a_hi {
        let base_exp = factor_exponents(a, &primes);
        for b in b_lo..=b_hi {
            let mut v = base_exp.clone();
            for e in &mut v {
                *e = (*e as usize * b) as u16;
            }
            seen.insert(v);
        }
    }
    seen.len()
}

fn main() {
    // Tiny example from the statement: 2..=5 for both a and b → 15 distinct terms
    assert_eq!(count_distinct(2, 5, 2, 5), 15);

    let total = count_distinct(2, 100, 2, 100);
    // println!("{}", total); // TODO: reveal locally
    println!("Distinct terms for 2<=a<=100, 2<=b<=100: [redacted]");
    let _ = total;
}
