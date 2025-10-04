// smallest_multiple_idiomatic.rs
// Prime-power construction of LCM(1..=n) with a simple sieve.
// Final print remains redacted per the near-finish policy.

fn sieve_primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2usize;
    while p * p <= limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    (2..=limit).filter(|&i| is_prime[i]).collect()
}

/// Highest power of `p` such that p^k <= n.
fn highest_power_leq(n: u128, p: u128) -> u128 {
    let mut value = p;
    while value * p <= n {
        value *= p;
    }
    value
}

fn lcm_via_primes(n: u128) -> u128 {
    let mut result = 1u128;
    for prime in sieve_primes(n as usize) {
        let power = highest_power_leq(n, prime as u128);
        result *= power;
    }
    result
}

fn main() {
    const N: u128 = 20;
    let result = lcm_via_primes(N);
    // println!("LCM via primes (1..={}) = {}", N, result); // TODO: reveal locally
    println!("LCM via primes (1..={}) = <compute locally>", N);

    // Cross-check with the fold-based approach on the sample range.
    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
    fn lcm(a: u128, b: u128) -> u128 {
        if a == 0 || b == 0 {
            return 0;
        }
        (a / gcd(a, b)) * b
    }
    fn lcm_range(n: u128) -> u128 {
        (1..=n).fold(1u128, |acc, x| lcm(acc, x))
    }

    assert_eq!(lcm_range(10), 2520);
    assert_eq!(lcm_via_primes(10), 2520);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_up_to_20() {
        assert_eq!(sieve_primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn highest_power_for_small_range() {
        assert_eq!(highest_power_leq(20, 2), 16);
        assert_eq!(highest_power_leq(20, 3), 9);
        assert_eq!(highest_power_leq(20, 5), 5);
    }

    #[test]
    fn lcm_matches_example() {
        assert_eq!(lcm_via_primes(10), 2520);
    }
}
