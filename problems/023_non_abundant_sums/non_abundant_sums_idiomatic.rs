// non_abundant_sums_idiomatic.rs
// An idiomatic and efficient Rust solution for Project Euler 23.
// Uses a divisor-sum sieve and boolean marking. Final answer redacted.

#[inline]
fn sum_proper_divisors_table(limit: usize) -> Vec<u32> {
    let mut spd = vec![0u32; limit + 1];
    for i in 1..=limit/2 {
        let mut j = i * 2;
        while j <= limit {
            spd[j] += i as u32;
            j += i;
        }
    }
    spd
}

fn main() {
    const LIMIT: usize = 28_123;

    // Precompute sum of proper divisors for 0..=LIMIT
    let spd = sum_proper_divisors_table(LIMIT);

    // Collect abundant numbers: d(n) > n
    let abundants: Vec<usize> = (1..=LIMIT).filter(|&n| spd[n] as usize > n).collect();

    // Mark all sums of two abundant numbers up to LIMIT
    let mut expressible = vec![false; LIMIT + 1];
    for (i_idx, &a) in abundants.iter().enumerate() {
        for &b in &abundants[i_idx..] {
            let s = a + b;
            if s > LIMIT { break; }
            expressible[s] = true;
        }
    }

    // Sum numbers that are not expressible
    let total: u64 = (1..=LIMIT)
        .filter(|&n| !expressible[n])
        .map(|n| n as u64)
        .sum();

    // println!("{}", total); // TODO: reveal locally
    println!("Sum of all positive integers <= {} not writable as sum of two abundant numbers = [redacted]", LIMIT);

    // Keep variable used to avoid warnings
    let _ = (total, abundants.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_abundant_is_12() {
        let spd = sum_proper_divisors_table(12);
        assert!(spd[12] > 12);
        assert!(spd[11] <= 11);
    }
}
