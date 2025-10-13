// digit_fifth_powers_simple.rs
// Project Euler 30 — Digit Fifth Powers (simple search with a tight upper bound).
// We'll precompute fifth powers of digits 0..9 and scan numbers from 2 up to 6*9^5.
// Final Euler answer intentionally not printed.

fn pow_u32(base: u32, exp: u32) -> u32 {
    let mut r = 1u32;
    for _ in 0..exp { r *= base; }
    r
}

fn sum_of_digit_powers(mut n: u32, pows: &[u32; 10]) -> u32 {
    let mut s = 0u32;
    if n == 0 { return pows[0]; }
    while n > 0 {
        let d = (n % 10) as usize;
        s += pows[d];
        n /= 10;
    }
    s
}

fn sum_numbers_equal_to_digit_powers(power: u32) -> u32 {
    // Upper bound argument:
    // For d digits, the largest sum of p-th powers is d * 9^p.
    // For p=5, 6 * 9^5 = 354_294 and 7 * 9^5 is still 6 digits,
    // so scanning up to 6*9^5 is sufficient.
    let nine_pow = pow_u32(9, power);
    let upper = 6 * nine_pow;

    // Precompute digit^power for digits 0..9
    let mut pows = [0u32; 10];
    for d in 0..=9u32 {
        pows[d as usize] = pow_u32(d, power);
    }

    let mut total = 0u32;
    for n in 2..=upper {
        if n == sum_of_digit_powers(n, &pows) {
            total += n;
        }
    }
    total
}

fn main() {
    // Sanity check from the statement for fourth powers: sum = 19316
    let fourth_sum = sum_numbers_equal_to_digit_powers(4);
    assert_eq!(fourth_sum, 19316);

    let fifth_sum = sum_numbers_equal_to_digit_powers(5);
    // println!("{}", fifth_sum); // TODO: reveal locally
    println!("Sum of all numbers equal to the sum of fifth powers of digits: [redacted]");
    let _ = fifth_sum;
}
