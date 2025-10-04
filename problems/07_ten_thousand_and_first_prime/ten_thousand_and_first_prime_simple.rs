// ten_thousand_and_first_prime_simple.rs
// Incremental prime generation using trial division by previously found primes.
// Final print redacted per the near-finish policy.

fn nth_prime_simple(n: usize) -> u64 {
    assert!(n >= 1, "n must be at least 1");
    let mut primes: Vec<u64> = Vec::new();
    let mut candidate: u64 = 2;

    while primes.len() < n {
        let mut is_prime = true;
        let limit = (candidate as f64).sqrt() as u64;

        for &p in &primes {
            if p > limit {
                break;
            }
            if candidate % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(candidate);
        }

        candidate += if candidate == 2 { 1 } else { 2 };
    }

    primes[n - 1]
}

fn main() {
    const N: usize = 10_001;
    let result = nth_prime_simple(N);
    println!("The {}th prime is <compute locally>", N);

    // Sanity check
    assert_eq!(nth_prime_simple(6), 13);
    let _ = result;
}
