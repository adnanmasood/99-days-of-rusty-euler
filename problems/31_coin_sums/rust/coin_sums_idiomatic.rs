// coin_sums_idiomatic.rs
// Project Euler 31 - Coin sums (idiomatic & robust Rust)
//
// This version wraps the logic in a pure function, uses consts for invariants,
// and adds small tests for edge cases. The dynamic programming structure is
// exactly the same as in the simple build, but expressed cleanly and readably.

const TARGET: usize = 200;
const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

#[inline]
fn count_ways(target: usize, coins: &[usize]) -> u64 {
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;

    // The coin-outer, amount-inner loop counts combinations without overcounting permutations.
    for &c in coins.iter() {
        for amount in c..=target {
            // Using checked_add could be an option for debug builds, but u64 is safe here.
            dp[amount] += dp[amount - c];
        }
    }
    dp[target]
}

fn main() {
    let result = count_ways(TARGET, &COINS);
    // println!("{}", result); // TODO: compute locally
    println!("Number of ways to make £2 is <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        assert_eq!(count_ways(0, &COINS), 1);
        assert_eq!(count_ways(0, &[]), 1); // only the empty selection
    }

    #[test]
    fn no_solution_when_odd_with_even_coins_only() {
        assert_eq!(count_ways(1, &[2, 4, 10]), 0);
    }

    #[test]
    fn one_way_with_coin_equal_to_target() {
        assert_eq!(count_ways(10, &[10]), 1);
    }
}
