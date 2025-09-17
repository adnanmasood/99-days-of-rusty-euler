// even_fibonacci_numbers_idiomatic.rs
// Two complementary paths:
//  1) Iterator pipeline that reads like the problem statement.
//  2) Fast recurrence that jumps only across even Fibonacci terms (O(log N)).

const LIMIT: u64 = 4_000_000;

// Policy: is this Fibonacci number "interesting" (i.e., even-valued)?
#[inline]
fn is_even(n: u64) -> bool { n % 2 == 0 }

// Programmatic improvement: use an iterator pipeline so intent is explicit
// and off-by-one risks are minimized.
fn sum_iter(limit: u64) -> u64 {
    use std::iter::successors;
    successors(Some((1u64, 2u64)), |(a, b)| Some((*b, a + b)))
        .map(|(a, _)| a)                      // project to the sequence 1, 2, 3, 5, 8, ...
        .take_while(|&n| n <= limit)          // respect the exclusive upper bound
        .filter(|&n| is_even(n))              // keep even-valued terms
        .sum::<u64>()                         // sum them
}

// Computational improvement: every third Fibonacci is even, and the even
// subsequence E_k satisfies E_{k+1} = 4*E_k + E_{k-1} with E_1 = 2, E_2 = 8.
fn sum_even_fast(limit: u64) -> u64 {
    if limit < 2 { return 0; }
    let (mut e1, mut e2) = (2u64, 8u64);
    let mut total = 0u64;

    if e1 <= limit { total += e1; }
    if e2 <= limit { total += e2; }

    loop {
        // Next even Fibonacci using the recurrence
        let e3 = 4 * e2 + e1;
        if e3 > limit { break; }
        total += e3;
        e1 = e2;
        e2 = e3;
    }
    total
}

fn main() {
    // Safe tiny checks (no Euler answer revealed):
    // println!("N=10 via iter  -> {}", sum_iter(10));       // would print 10
    // println!("N=10 via fast  -> {}", sum_even_fast(10));   // would print 10

    let _sum1 = sum_iter(LIMIT);
    let _sum2 = sum_even_fast(LIMIT);

    // Keep the final number hidden in the series post.
    // println!("{}", _sum2); // TODO: uncomment locally
    println!("TODO: run locally to compute final answers (iter & fast) for LIMIT={}", LIMIT);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn methods_agree_small() {
        // Quick consistency check; avoids revealing any particular number.
        for bound in [10u64, 100, 10_000] {
            assert_eq!(sum_iter(bound), sum_even_fast(bound));
        }
    }
}
