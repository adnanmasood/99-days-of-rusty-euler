// multiples_of_3_or_5_idiomatic.rs
// O(1) solution using arithmetic series and inclusion–exclusion.
// Keeps the final answer redacted per the "near-finish" rule.

#[inline]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[inline]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// Sum of all positive multiples of k strictly below `limit`.
fn sum_of_multiples_below(limit: u64, k: u64) -> u128 {
    assert!(k > 0, "k must be positive");
    if limit <= 1 {
        return 0;
    }

    let m = (limit - 1) / k; // m_k = floor((N-1)/k)
    let m128 = m as u128;
    let k128 = k as u128;
    k128 * m128 * (m128 + 1) / 2 // k * m * (m + 1) / 2
}

fn sum_3_or_5(limit: u64) -> u128 {
    let s3 = sum_of_multiples_below(limit, 3);
    let s5 = sum_of_multiples_below(limit, 5);
    let s15 = sum_of_multiples_below(limit, lcm(3, 5));
    s3 + s5 - s15
}

fn main() {
    const LIMIT: u64 = 1000;
    let total = sum_3_or_5(LIMIT);
    // println!("Answer: {}", total); // TODO: reveal locally
    println!("S = S3 + S5 - S15  (evaluate locally)");
    let _ = total; // keep the computation live without printing the answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_under_10() {
        // Below 10 -> 23 (3 + 5 + 6 + 9)
        assert_eq!(sum_3_or_5(10), 23);
    }
}
