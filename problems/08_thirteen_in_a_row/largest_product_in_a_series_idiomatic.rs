// largest_product_in_a_series_idiomatic.rs
// O(n) sliding window with zero-aware rolling product.

use std::cmp::max;

const N1000: &str = r#"73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450"#;

fn digits_from_block(s: &str) -> Vec<u8> {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

/// Return the maximum product of any length-k window over the digit slice.
/// Uses a rolling product that skips across zeros in O(n).
pub fn max_product_k(d: &[u8], k: usize) -> u128 {
    if k == 0 || d.is_empty() || k > d.len() {
        return 0;
    }

    let mut best: u128 = 0;
    let mut prod: u128 = 1;
    let mut zeros: usize = 0;

    for i in 0..d.len() {
        let x = d[i] as u128;
        if x == 0 {
            zeros += 1;
        } else {
            prod *= x;
        }

        if i >= k {
            let y = d[i - k] as u128;
            if y == 0 {
                zeros -= 1;
            } else {
                prod /= y;
            }
        }

        if i + 1 >= k && zeros == 0 {
            best = max(best, prod);
        }
    }

    best
}

fn main() {
    let digits = digits_from_block(N1000);
    let best13 = max_product_k(&digits, 13);
    println!("best product (13) = {}", best13);

    // Example check:
    assert_eq!(max_product_k(&digits, 4), 5832);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_window_of_four() {
        let digits = digits_from_block(N1000);
        assert_eq!(max_product_k(&digits, 4), 5832);
    }

    #[test]
    fn puzzle_answer() {
        let digits = digits_from_block(N1000);
        assert_eq!(max_product_k(&digits, 13), 23514624000);
    }
}
