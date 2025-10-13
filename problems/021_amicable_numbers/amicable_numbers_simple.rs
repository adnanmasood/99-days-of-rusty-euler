// amicable_numbers_simple.rs
// A clear, loop-first solution for Project Euler 21 (Amicable Numbers).
// Intentionally avoids printing the final Euler answer; see the println! near the end.

fn sum_proper_divisors(n: u64) -> u64 {
    // Proper divisors are numbers < n that divide n evenly.
    // We handle n <= 1 up front: by definition, there are no proper divisors.
    if n <= 1 { return 0; }

    // Start with 1, which divides every n > 1.
    let mut sum = 1u64;

    // Check possible divisors up to sqrt(n).
    // For each i that divides n, add i and its paired divisor n/i.
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            sum += i;
            let d = n / i;
            if d != i {
                sum += d; // add the complementary divisor if it's different
            }
        }
        i += 1;
    }
    sum
}

fn is_amicable(a: u64) -> Option<u64> {
    // b is the sum of proper divisors of a
    let b = sum_proper_divisors(a);
    if b != a && sum_proper_divisors(b) == a {
        Some(b)
    } else {
        None
    }
}

fn sum_amicable_under(limit: u64) -> u64 {
    let mut total = 0u64;
    for a in 2..limit {
        if let Some(_b) = is_amicable(a) {
            total += a; // we only include a if a < limit, as required by the problem
        }
    }
    total
}

fn main() {
    const LIMIT: u64 = 10_000;

    // Quick sanity check from the statement: 220 and 284 form an amicable pair.
    // (We do not print this; just compute to keep the compiler happy.)
    let _check_220 = sum_proper_divisors(220);
    let _check_284 = sum_proper_divisors(284);
    debug_assert_eq!(_check_220, 284);
    debug_assert_eq!(_check_284, 220);

    let sum = sum_amicable_under(LIMIT);

    // Keep the final answer hidden for the 99‑day series.
    // println!("{}", sum); // TODO: reveal locally
    println!("Sum of amicable numbers under {} = [redacted]", LIMIT);

    // Prevent unused warning in non-debug builds
    let _ = sum;
}
