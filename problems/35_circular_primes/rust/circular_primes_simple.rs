// circular_primes_simple.rs
// Project Euler 35 - Circular Primes (simple, didactic version)
//
// Count primes below one million such that all rotations of their digits
// are also prime. Example: 197 -> 971 -> 719 are all prime.
//
// This build focuses on clarity: a sieve for primality, a digit-rotation
// helper, and a predicate `is_circular_prime`. We also prune multi-digit
// candidates containing any even digit or 5 (since some rotation would end
// with that digit and be composite).
//
// No spoilers: the main prints a placeholder for the count below one million.
// It does, however, print the known small example count below 100 (= 13).

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

fn pow10(k: usize) -> usize {
    let mut p = 1usize;
    for _ in 0..k { p *= 10; }
    p
}

fn digit_count(mut n: usize) -> usize {
    if n == 0 { return 1; }
    let mut c = 0usize;
    while n > 0 { n /= 10; c += 1; }
    c
}

fn has_forbidden_digits(n: usize) -> bool {
    // If the number has any of {0,2,4,5,6,8}, then some rotation
    // will end with that digit, making it composite. Single-digit
    // primes 2 and 5 are allowed but handled separately.
    let mut x = n;
    while x > 0 {
        match x % 10 {
            0 | 2 | 4 | 5 | 6 | 8 => return true,
            _ => {}
        }
        x /= 10;
    }
    false
}

fn is_circular_prime(n: usize, is_prime: &Vec<bool>) -> bool {
    if n < 10 {
        return is_prime[n];
    }
    if has_forbidden_digits(n) { return false; }

    let k = digit_count(n);
    let p10 = pow10(k - 1);
    let mut r = n;
    for _ in 0..k {
        if !is_prime[r] { return false; }
        // rotate right by 1: last digit to front
        let last = r % 10;
        r = last * p10 + r / 10;
    }
    true
}

fn count_circular_primes(limit: usize) -> usize {
    let is_prime = sieve(limit);
    let mut count = 0usize;
    for n in 2..limit {
        if is_circular_prime(n, &is_prime) {
            count += 1;
        }
    }
    count
}

fn main() {
    let under_100 = count_circular_primes(100);
    println!("circular primes below 100 = {} (expected 13)", under_100);

    let _under_million = count_circular_primes(1_000_000);
    // println!("{}", _under_million); // TODO: reveal locally
    println!("circular primes below one million = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_197_is_circular() {
        let is_prime = sieve(1_000_000);
        assert!(is_circular_prime(197, &is_prime));
    }

    #[test]
    fn count_below_100_is_13() {
        assert_eq!(count_circular_primes(100), 13);
    }
}
