// circular_primes_idiomatic.rs
// Project Euler 35 - Circular Primes (idiomatic & robust Rust)
//
// This version keeps the same algorithm but expresses it with small,
// well-named helpers and early pruning. We rely on a classic sieve and
// an allocation-free rotation loop.

#[inline]
fn sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit];
    if limit > 0 { is_prime[0] = false; }
    if limit > 1 { is_prime[1] = false; }
    let mut p = 2usize;
    while p * p < limit {
        if is_prime[p] {
            let mut m = p * p;
            while m < limit {
                is_prime[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    is_prime
}

#[inline]
fn pow10(exp: usize) -> usize {
    let mut acc = 1usize;
    for _ in 0..exp { acc *= 10; }
    acc
}

#[inline]
fn digit_count(mut n: usize) -> usize {
    if n == 0 { return 1; }
    let mut k = 0;
    while n > 0 { n /= 10; k += 1; }
    k
}

#[inline]
fn has_forbidden_digits(mut n: usize) -> bool {
    while n > 0 {
        match n % 10 {
            0 | 2 | 4 | 5 | 6 | 8 => return true,
            _ => {}
        }
        n /= 10;
    }
    false
}

#[inline]
fn is_circular_prime(n: usize, sieve: &Vec<bool>) -> bool {
    if n < 10 {
        return sieve[n];
    }
    if has_forbidden_digits(n) { return false; }

    let k = digit_count(n);
    let p10 = pow10(k - 1);
    let mut r = n;
    for _ in 0..k {
        if !sieve[r] { return false; }
        let last = r % 10;
        r = last * p10 + r / 10; // right-rotate by one digit
    }
    true
}

pub fn count_circular_primes(limit: usize) -> usize {
    let sieve = sieve(limit);
    (2..limit).filter(|&n| is_circular_prime(n, &sieve)).count()
}

fn main() {
    let small = count_circular_primes(100);
    println!("below 100 = {} (should be 13)", small);

    let ans = count_circular_primes(1_000_000);
    // println!("{}", ans); // TODO: reveal locally
    println!("below one million = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let primes = sieve(1_000_000);
        assert!(is_circular_prime(197, &primes));
    }

    #[test]
    fn count_small() {
        assert_eq!(count_circular_primes(100), 13);
    }
}
