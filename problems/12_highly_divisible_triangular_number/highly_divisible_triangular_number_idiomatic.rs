// highly_divisible_triangular_number_idiomatic.rs
// Project Euler 12 - Highly Divisible Triangular Number
// A faster, more idiomatic approach that factors n and n+1 separately and uses a tiny sieve.
const TARGET: u64 = 500;
fn sieve(limit: usize) -> Vec<u64> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    let mut p = 2usize;
    while p * p <= limit {
        if is_prime[p] {
            let mut q = p * p;
            while q <= limit {
                is_prime[q] = false;
                q += p;
            }
        }
        p += 1;
    }
    is_prime.iter().enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| i as u64)
        .collect()
}
fn count_divisors_with_primes(mut n: u64, primes: &[u64]) -> u64 {
    if n == 0 { return 0; }
    let mut count = 1u64;
    for &p in primes {
        if p * p > n { break; }
        if n % p == 0 {
            let mut exp = 0u64;
            while n % p == 0 {
                n /= p;
                exp += 1;
            }
            count *= exp + 1;
        }
    }
    if n > 1 { count *= 2; }
    count
}
fn triangular_divisors(n: u64, primes: &[u64]) -> u64 {
    // T_n = n*(n+1)/2 ; factor the even part out to keep the two factors coprime
    if n % 2 == 0 {
        // n/2 and n+1 are coprime
        count_divisors_with_primes(n / 2, primes) * count_divisors_with_primes(n + 1, primes)
    } else {
        // n and (n+1)/2 are coprime
        count_divisors_with_primes(n, primes) * count_divisors_with_primes((n + 1) / 2, primes)
    }
}
fn main() {
    // sqrt of the largest T_n we'll see is safely below 100_000 for this problem;
    // a small sieve is more than enough.
    let primes = sieve(100_000);
    let mut n: u64 = 1;
    loop {
        let d = triangular_divisors(n, &primes);
        if d > TARGET {
            let tri = n * (n + 1) / 2;
            // println!("n = {}, T_n = {}, divisors = {}", n, tri, d); // TODO
            println!("Computed T_n exceeding {} divisors. (Redacted.)", TARGET);
            break;
        }
        n += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t7_has_over_five_divisors() {
        let primes = sieve(1000);
        let n = 7u64;
        let tri = n*(n+1)/2;
        assert_eq!(tri, 28);
        let d = triangular_divisors(n, &primes);
        assert!(d > 5, "T_7 should have over five divisors");
    }
}
