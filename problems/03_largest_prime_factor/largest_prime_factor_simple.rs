// largest_prime_factor_simple.rs
// Simple trial-division factorization; final answer redacted.
/// Return the largest prime factor of n via trial division.
fn largest_prime_factor(mut n: u128) -> u128 {
    let mut largest: u128 = 0;

    // Remove factor 2 first to make the main loop odd-only.
    while n % 2 == 0 {
        largest = 2;
        n /= 2;
    }

    // Now try odd factors up to sqrt(n).
    let mut f: u128 = 3;
    // Use f <= n / f to avoid f*f overflow.
    while f <= n / f {
        if n % f == 0 {
            largest = f;
            while n % f == 0 {
                n /= f;
            }
        }
        f += 2;
    }

    // If anything remains, it's prime and larger than any factor seen.
    if n > 1 {
        largest = largest.max(n);
    }

    largest
}

fn main() {
    const N: u128 = 600_851_475_143;
    let ans = largest_prime_factor(N);
    // println!("Largest prime factor of {} is {}", N, ans); // TODO: reveal locally
    println!("Largest prime factor of {} is <compute locally>", N);
    let _ = ans; // silence 'unused variable' if print is commented out

    // Sanity check from the statement:
    assert_eq!(largest_prime_factor(13_195), 29);
}
