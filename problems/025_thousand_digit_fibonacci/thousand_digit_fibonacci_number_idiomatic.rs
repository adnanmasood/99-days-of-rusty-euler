// thousand_digit_fibonacci_number_idiomatic.rs
// Project Euler 25 — idiomatic version with math: use logs to jump directly.
// Final Euler answer intentionally not printed.

fn index_for_digits_log10(d: u32) -> u32 {
    // We want minimal n such that number of digits of F_n is >= d.
    // For large n, F_n ≈ phi^n / sqrt(5).
    // digits(F_n) = floor(log10(F_n)) + 1.
    // So require: n*log10(phi) - log10(sqrt(5)) >= d - 1.
    let phi = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
    let log10_phi = phi.log10();
    let log10_sqrt5 = 5.0_f64.sqrt().log10();
    let n = ((d as f64 - 1.0 + log10_sqrt5) / log10_phi).ceil();
    n as u32
}

fn main() {
    // Sanity check with d=3; F_12 is the first 3-digit Fibonacci number.
    let n3 = index_for_digits_log10(3);
    debug_assert_eq!(n3, 12);

    let n1000 = index_for_digits_log10(1000);
    // println!("{}", n1000); // TODO: reveal locally
    println!("Index of first Fibonacci term with 1000 digits = [redacted]");
    let _ = n1000;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_small_threshold() {
        assert_eq!(index_for_digits_log10(1), 1);
        assert_eq!(index_for_digits_log10(2), 7);  // F7 = 13 -> 2 digits (first time)
        assert_eq!(index_for_digits_log10(3), 12); // from statement
    }
}
