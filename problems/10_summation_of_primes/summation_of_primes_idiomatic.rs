// summation_of_primes_idiomatic.rs
// Sieve of Eratosthenes with u128 accumulation. Final print redacted.

const LIMIT: usize = 2_000_000; // sum primes strictly below this

pub fn sieve_sum(limit: usize) -> u128 {
    if limit <= 2 {
        return 0;
    }
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2usize;
    while p * p < limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple < limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    let mut sum: u128 = 0;
    for i in 2..limit {
        if is_prime[i] {
            sum += i as u128;
        }
    }
    sum
}

fn main() {
    let total = sieve_sum(LIMIT);
    // println!("{}", total); // TODO: reveal locally
    println!("sum(primes < {}) = <compute locally>", LIMIT);
    // Example from the statement: below 10 => 17
    assert_eq!(sieve_sum(10), 17);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_below_10() {
        assert_eq!(sieve_sum(10), 17);
    }

    #[test]
    fn sum_below_20() {
        assert_eq!(sieve_sum(20), 77);
    }
}
