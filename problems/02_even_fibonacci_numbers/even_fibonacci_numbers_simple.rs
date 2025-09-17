// even_fibonacci_numbers_simple.rs
// Simple, direct scan of Fibonacci numbers up to `limit`.
// Intentionally avoids printing the final Euler answer in this series post.

fn sum_even_fib_naive(limit: u64) -> u64 {
    // Fibonacci starts as 1, 2 per the problem statement.
    let (mut a, mut b) = (1u64, 2u64);
    let mut total = 0u64;

    // Traverse until we pass the limit (exclusive upper bound).
    while a <= limit {
        if a % 2 == 0 {
            total += a; // accumulate only even-valued terms
        }
        // advance the pair (a, b) -> (b, a+b)
        let next = a + b;
        a = b;
        b = next;
    }
    total
}

fn main() {
    // Tiny check (does not reveal the Euler answer):
    // println!("Check N=10 -> {}", sum_even_fib_naive(10)); // would print 10

    let limit = 4_000_000u64;
    // Keep the series result hidden:
    // println!("{}", sum_even_fib_naive(limit)); // TODO: uncomment locally
    println!("TODO: run locally to compute the final answer for limit = {}", limit);
}
