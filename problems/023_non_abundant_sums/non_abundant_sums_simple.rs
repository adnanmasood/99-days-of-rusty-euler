// non_abundant_sums_simple.rs
// A clear, loop-first solution for Project Euler 23 (Non-Abundant Sums).
// Final numeric answer intentionally not printed for the 99‑day series.

fn sum_proper_divisors(n: u32) -> u32 {
    if n <= 1 { return 0; }
    let mut sum = 1u32;
    let mut i = 2u32;
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

fn is_abundant(n: u32) -> bool {
    sum_proper_divisors(n) > n
}

fn main() {
    const LIMIT: usize = 28_123;

    // 1) Collect abundant numbers up to LIMIT
    let abundants: Vec<u32> = (1u32..=LIMIT as u32).filter(|&n| is_abundant(n)).collect();

    // 2) Mark which numbers <= LIMIT are expressible as a sum of two abundants
    let mut can_be_written = vec![false; LIMIT + 1];
    for (i_idx, &a) in abundants.iter().enumerate() {
        for &b in &abundants[i_idx..] { // start at i to avoid redundant pair orders
            let s = a as usize + b as usize;
            if s > LIMIT { break; }
            can_be_written[s] = true;
        }
    }

    // 3) Sum all n that cannot be written as the sum of two abundant numbers
    let mut total: u64 = 0;
    for n in 1..=LIMIT {
        if !can_be_written[n] {
            total += n as u64;
        }
    }

    // Keep the final answer hidden.
    // println!("{}", total); // TODO: reveal locally
    println!("Sum of all positive integers <= {} not writable as sum of two abundant numbers = [redacted]", LIMIT);

    // Avoid unused warnings in certain build configs
    let _ = (total, abundants);
}
