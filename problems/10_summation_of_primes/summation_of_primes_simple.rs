// summation_of_primes_simple.rs
// A straightforward prime test with trial division up to sqrt(n).
// We sum all primes strictly less than LIMIT. Final print redacted per policy.

const LIMIT: u64 = 2_000_000;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d: u64 = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn sum_primes_below(limit: u64) -> u128 {
    let mut total: u128 = 0;
    let mut n: u64 = 2;
    while n < limit {
        if is_prime(n) {
            total += n as u128;
        }
        n += 1;
    }
    total
}

fn main() {
    let total = sum_primes_below(LIMIT);
    // println!("{}", total); // TODO: reveal locally
    println!("sum(primes < {}) = <compute locally>", LIMIT);
    // tiny statement check: primes below 10 are 2, 3, 5, 7 -> sum 17
    assert_eq!(sum_primes_below(10), 17);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_below_10() {
        assert_eq!(sum_primes_below(10), 17);
    }
}
