// ten_thousand_and_first_prime_idiomatic.rs
// Sieve of Eratosthenes with a prime-number-theorem upper bound for p_n.
// Final value redacted.

/// Upper bound for the nth prime using Dusart/Rosser-style estimates:
/// For n >= 6, p_n < n * (ln n + ln ln n).
fn upper_bound_nth_prime(n: usize) -> usize {
    if n < 6 {
        15
    } else {
        let nn = n as f64;
        let ub = nn * (nn.ln() + nn.ln().ln());
        ub.ceil() as usize + 10 // small safety margin
    }
}

fn sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    if limit >= 0 {
        is_prime[0] = false;
    }
    if limit >= 1 {
        is_prime[1] = false;
    }
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

    let mut primes = Vec::new();
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}

pub fn nth_prime(n: usize) -> usize {
    assert!(n >= 1, "n must be at least 1");
    let mut limit = upper_bound_nth_prime(n);
    loop {
        let primes = sieve(limit);
        if primes.len() >= n {
            return primes[n - 1];
        }
        // If the bound was too small, grow it and try again.
        limit *= 2;
    }
}

fn main() {
    const N: usize = 10_001;
    let p = nth_prime(N);
    // println!("p_{} = {}", N, p); // TODO: reveal locally
    println!("p_{} = <compute locally>", N);
    let _ = p;
    // Example from the statement:
    assert_eq!(nth_prime(6), 13);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_primes() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(6), 13);
    }
}
