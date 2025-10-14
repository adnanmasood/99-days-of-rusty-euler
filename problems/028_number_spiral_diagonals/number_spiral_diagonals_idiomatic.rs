// number_spiral_diagonals_idiomatic.rs
// Project Euler 28 — closed-form computation for the diagonal sum.
// For an odd side length n, with m = (n - 1) / 2, each layer k (1..=m) adds:
// layer_sum(k) = 16*k^2 + 4*k + 4
// So:
// S(n) = 1 + sum_{k=1..m} (16k^2 + 4k + 4)
//     = 1 + 16*sum k^2 + 4*sum k + 4*m
// Final Euler answer intentionally not printed.

fn diag_sum(n: u128) -> u128 {
    assert!(n % 2 == 1, "n must be odd");
    let m = (n - 1) / 2;
    let sum_k2 = m * (m + 1) * (2 * m + 1) / 6;
    let sum_k  = m * (m + 1) / 2;
    1 + 16 * sum_k2 + 4 * sum_k + 4 * m
}

fn diag_sum_closed_form_n(n: u128) -> u128 {
    // Equivalent one-liner in terms of n:
    // S(n) = (4n^3 + 3n^2 + 8n - 9) / 6
    (4*n*n*n + 3*n*n + 8*n - 9) / 6
}

fn main() {
    assert_eq!(diag_sum(1), 1);
    assert_eq!(diag_sum(3), 25);
    assert_eq!(diag_sum(5), 101);
    // Cross-check the two formulas on a few values
    for &n in &[1u128, 3, 5, 7, 9, 11, 101, 1001] {
        assert_eq!(diag_sum(n), diag_sum_closed_form_n(n));
    }

    let n: u128 = 1001;
    let ans = diag_sum(n);
    // println!("{}", ans); // TODO: reveal locally
    println!("Sum of diagonals for {}x{} spiral = [redacted]", n, n);
    let _ = ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_cases() {
        assert_eq!(diag_sum(1), 1);
        assert_eq!(diag_sum(3), 25);
        assert_eq!(diag_sum(5), 101);
    }
}
