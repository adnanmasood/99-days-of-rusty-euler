// quadratic_primes_simple.rs
// Project Euler 27 — Quadratic Primes (simple search).
// We scan all a,b in the given bounds and count consecutive primes for
// n starting at 0. Final Euler answer intentionally not printed.

fn is_prime(n: i64) -> bool {
    if n <= 1 { return false; }
    if n % 2 == 0 { return n == 2; }
    let mut d = 3i64;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}

fn prime_run_len(a: i64, b: i64) -> i64 {
    // Count consecutive n >= 0 for which n^2 + a*n + b is prime.
    let mut n = 0i64;
    loop {
        let val = n * n + a * n + b;
        if !is_prime(val) { break; }
        n += 1;
    }
    n
}

fn main() {
    // Sanity checks from the statement:
    // n^2 + n + 41 yields 40 primes for n=0..39
    assert_eq!(prime_run_len(1, 41), 40);
    // n^2 - 79n + 1601 yields 80 primes for n=0..79
    assert_eq!(prime_run_len(-79, 1601), 80);

    let mut best_a = 0i64;
    let mut best_b = 0i64;
    let mut best_len = 0i64;

    for a in -999i64..=999 {
        for b in -1000i64..=1000 {
            if b <= 1 || !is_prime(b) { continue; } // f(0) = b must be prime (>1)
            let len = prime_run_len(a, b);
            if len > best_len {
                best_len = len;
                best_a = a;
                best_b = b;
            }
        }
    }

    let product = best_a * best_b;
    // println!("{}", product); // TODO: reveal locally
    println!("Best (a,b,len,product) = [redacted]");

    // keep variables used
    let _ = (product, best_a, best_b, best_len);
}
