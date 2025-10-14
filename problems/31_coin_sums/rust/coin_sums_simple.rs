// coin_sums_simple.rs
// Project Euler 31 - Coin sums (simple, didactic version)
//
// Goal: Count the number of ways to make a target amount from given coin denominations.
// We'll use the classic 1-D dynamic programming approach where order of coins does not matter.
//
// NOTE: In keeping with the no-spoiler policy for the series, we DO NOT print the final answer here.
// Replace the placeholder print with a local computation if you want to see the number on your machine.

const TARGET: usize = 200; // 200 pence = £2
const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn count_ways(target: usize, coins: &[usize]) -> u64 {
    // dp[a] = number of ways to make 'a' pence using the processed coins so far
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1; // one way to make zero: choose no coins

    // Outer loop over coins ensures combinations (order-free counting)
    for &c in coins {
        // For each amount at least 'c', add ways that end with coin 'c'
        for amount in c..=target {
            dp[amount] += dp[amount - c];
        }
    }
    dp[target]
}

fn main() {
    let ways = count_ways(TARGET, &COINS);
    // println!("{}", ways); // TODO: reveal locally if you want
    println!("ways[{}] = <redacted>", TARGET);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_amount_has_one_way() {
        assert_eq!(count_ways(0, &COINS), 1);
    }

    #[test]
    fn single_coin_one_always_one_way() {
        // With only the 1p coin, there's exactly one way to make any target.
        let coins = [1usize];
        assert_eq!(count_ways(5, &coins), 1);
        assert_eq!(count_ways(37, &coins), 1);
    }

    #[test]
    fn impossible_with_even_coin_only() {
        // With only coin 2, odd targets are impossible.
        let coins = [2usize];
        assert_eq!(count_ways(3, &coins), 0);
        assert_eq!(count_ways(4, &coins), 1); // 2+2
    }
}
