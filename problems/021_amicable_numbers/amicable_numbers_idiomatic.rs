// amicable_numbers_idiomatic.rs
// A more idiomatic and efficient Rust solution for Project Euler 21.
// Uses a "divisor sieve" to precompute sum of proper divisors for all n < LIMIT.
// Final numeric answer intentionally not printed.

#[inline]
fn sum_proper_divisors_unbounded(n: u64) -> u64 {
    if n <= 1 { return 0; }
    let mut sum = 1u64;
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            sum += i;
            let d = n / i;
            if d != i { sum += d; }
        }
        i += 1;
    }
    sum
}

fn sum_proper_divisors_table(limit: usize) -> Vec<u64> {
    // spd[x] will hold sum of proper divisors of x, for 0 <= x < limit.
    let mut spd = vec![0u64; limit];
    // For each i, add i to all multiples j = 2*i, 3*i, ...
    for i in 1..limit {
        let mut j = i * 2;
        while j < limit {
            spd[j] += i as u64;
            j += i;
        }
    }
    spd
}

fn main() {
    const LIMIT: usize = 10_000;
    let spd = sum_proper_divisors_table(LIMIT);

    let sum: u64 = (2..LIMIT)
        .filter_map(|a| {
            let b = spd[a] as usize;
            if b != a {
                // If b is inside table, use it; otherwise compute on the fly.
                let db = if b < LIMIT { spd[b] } else { sum_proper_divisors_unbounded(b as u64) };
                if db == a as u64 { Some(a as u64) } else { None }
            } else {
                None
            }
        })
        .sum();

    // println!("{}", sum); // TODO: reveal locally
    println!("Sum of amicable numbers under {} = [redacted]", LIMIT);

    // Avoid unused warning
    let _ = sum;
}
