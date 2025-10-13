// distinct_powers_idiomatic.rs
// Project Euler 29 — Distinct Powers (idiomatic, with helpers and tests).
// Idea: represent a^b by the vector of prime exponents of a, scaled by b.
// Equality of vectors <=> equality of numbers, so we use a HashSet<Vec<u16>>.
// Final Euler answer intentionally not printed.

use std::collections::HashSet;

fn primes_up_to(n: usize) -> Vec<usize> {
    let mut is_p = vec![true; n + 1];
    is_p[0] = false; is_p[1] = false;
    for p in 2..=((n as f64).sqrt() as usize) {
        if is_p[p] {
            for m in (p*p..=n).step_by(p) {
                is_p[m] = false;
            }
        }
    }
    (2..=n).filter(|&x| is_p[x]).collect()
}

fn exponents_of(mut a: usize, primes: &[usize]) -> Vec<u16> {
    let mut v = vec![0u16; primes.len()];
    for (i, &p) in primes.iter().enumerate() {
        if p * p > a { break; }
        while a % p == 0 {
            v[i] += 1;
            a /= p;
        }
    }
    if a > 1 {
        let i = primes.iter().position(|&q| q == a).expect("prime must be in list");
        v[i] += 1;
    }
    v
}

fn distinct_count(a_range: std::ops::RangeInclusive<usize>, b_range: std::ops::RangeInclusive<usize>) -> usize {
    let &a_hi = a_range.end();
    let primes = primes_up_to(a_hi);
    let mut set: HashSet<Vec<u16>> = HashSet::new();

    for a in a_range {
        let base = exponents_of(a, &primes);
        for b in b_range.clone() {
            // Scale the exponent vector by b
            let mut v = base.clone();
            for e in &mut v { *e = (*e as usize * b) as u16; }
            set.insert(v);
        }
    }
    set.len()
}

fn main() {
    assert_eq!(distinct_count(2..=5, 2..=5), 15);
    let total = distinct_count(2..=100, 2..=100);
    // println!("{}", total); // TODO: reveal locally
    println!("Distinct terms for 2<=a<=100, 2<=b<=100: [redacted]");
    let _ = total;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tiny_example() {
        assert_eq!(distinct_count(2..=5, 2..=5), 15);
    }
}
