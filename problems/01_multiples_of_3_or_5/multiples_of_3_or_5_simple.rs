// multiples_of_3_or_5_simple.rs
// Naive scan with clear comments (no spoiler: final print redacted).

fn is_multiple_of_3_or_5(n: u64) -> bool {
    // A number is a multiple of 3 or 5 if it divides evenly by 3 or 5.
    n % 3 == 0 || n % 5 == 0
}

fn sum_multiples_scan(limit: u64) -> u64 {
    // Sum all integers k with 1 <= k < limit that are multiples of 3 or 5.
    let mut total: u64 = 0;
    // The range 1..limit is exclusive on the right in Rust.
    for k in 1..limit {
        if is_multiple_of_3_or_5(k) {
            total += k;
        }
    }
    total
}

fn main() {
    const LIMIT: u64 = 1000; // upper bound, exclusive
    let result = sum_multiples_scan(LIMIT);
    // println!("Sum for {} = {}", LIMIT, result); // TODO: reveal locally
    println!("Sum for {} = <compute locally>", LIMIT);
    // Sanity check using the example from the problem statement:
    // Below 10, the multiples are 3, 5, 6, 9; they sum to 23.
    assert_eq!(sum_multiples_scan(10), 23);
}
