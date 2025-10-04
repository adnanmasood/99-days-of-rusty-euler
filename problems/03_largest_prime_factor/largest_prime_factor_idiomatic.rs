// largest_prime_factor_idiomatic.rs
// Idiomatic, robust factorization with small helpers and overflow-safe bounds.

#[inline]
fn remove_factor(n: &mut u128, p: u128) -> bool {
    if *n % p != 0 {
        return false;
    }
    while *n % p == 0 {
        *n /= p;
    }
    true
}

/// Largest prime factor using trial division with pruning.
pub fn largest_prime_factor(mut n: u128) -> u128 {
    let mut best: u128 = 0;

    if remove_factor(&mut n, 2) {
        best = 2;
    }

    let mut f: u128 = 3;
    // Loop only while f <= sqrt(n); bound shrinks as n shrinks.
    while f <= n / f {
        if remove_factor(&mut n, f) {
            best = f;
        }
        f += 2;
    }

    if n > 1 {
        best = best.max(n);
    }

    best
}

fn main() {
    const N: u128 = 600_851_475_143;
    let result = largest_prime_factor(N);
    // println!("Answer: {}", result); // TODO: reveal locally
    println!("result = max(best, remaining_n)  // evaluate locally");
    let _ = result;

    // Example from statement:
    assert_eq!(largest_prime_factor(13_195), 29);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_13195() {
        assert_eq!(largest_prime_factor(13_195), 29);
    }

    #[test]
    fn small_primes() {
        assert_eq!(largest_prime_factor(2), 2);
        assert_eq!(largest_prime_factor(29), 29);
        assert_eq!(largest_prime_factor(97), 97);
    }

    #[test]
    fn powers() {
        assert_eq!(largest_prime_factor(2u128.pow(10)), 2);
        assert_eq!(largest_prime_factor(3u128.pow(8)), 3);
    }
}
