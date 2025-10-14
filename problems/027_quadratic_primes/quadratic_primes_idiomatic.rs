// quadratic_primes_idiomatic.rs
// Project Euler 27 — an idiomatic and optimized solution.
// Observations:
//  - For n=0, the polynomial evaluates to b; so b must be prime and > 1.
//  - For b != 2 (i.e., b odd), 1 + a + b must be prime. That forces 'a' to be odd.
// We pre-sieve primes up to 200_000 (more than enough for values encountered).
// Final Euler answer intentionally not printed.

fn sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit >= 0 { is_prime[0] = false; }
    if limit >= 1 { is_prime[1] = false; }
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
    is_prime
}

#[inline]
fn is_prime_basic(n: i64) -> bool {
    if n <= 1 { return false; }
    if n % 2 == 0 { return n == 2; }
    let mut d = 3i64;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

fn prime_run_len(a: i64, b: i64) -> i64 {
    let mut n = 0i64;
    loop {
        let val = n * n + a * n + b;
        if !is_prime_basic(val) { break; }
        n += 1;
    }
    n
}

fn main() {
    // Examples from the statement as unit-like sanity checks
    assert_eq!(prime_run_len(1, 41), 40);
    assert_eq!(prime_run_len(-79, 1601), 80);

    // Candidate b's are primes <= 1000 (positive)
    let mut b_candidates: Vec<i64> = Vec::new();
    for b in 2..=1000 {
        if is_prime_basic(b) {
            b_candidates.push(b as i64);
        }
    }

    let mut best = (0i64, 0i64, 0i64); // (len, a, b)
    for &b in &b_candidates {
        let a_iter: Box<dyn Iterator<Item = i64>> = if b != 2 {
            Box::new((-999..=999).filter(|a| a % 2 != 0))
        } else {
            Box::new(-999..=999)
        };
        for a in a_iter {
            let len = prime_run_len(a, b);
            if len > best.0 {
                best = (len, a, b);
            }
        }
    }

    let product = best.1 * best.2;
    // println!("{}", product); // TODO: reveal locally
    println!("Best (a,b,len,product) = [redacted]");
    let _ = product;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples_hold() {
        assert_eq!(prime_run_len(1, 41), 40);
        assert_eq!(prime_run_len(-79, 1601), 80);
    }
}
