// sum_square_difference_simple.rs
// Direct formulas with u128 arithmetic. Final print redacted.

/// S(n) = 1 + 2 + ... + n
fn sum_to(n: u128) -> u128 {
    n * (n + 1) / 2
}

/// Q(n) = 1^2 + 2^2 + ... + n^2
fn sum_of_squares(n: u128) -> u128 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn square(x: u128) -> u128 {
    x * x
}

fn difference(n: u128) -> u128 {
    let s = sum_to(n);
    let q = sum_of_squares(n);
    square(s) - q
}

fn main() {
    const N: u128 = 100;
    let d = difference(N);
    // println!("difference for {} = {}", N, d); // TODO: reveal locally
    println!("difference for {} = <compute locally>", N);
    // Statement check for n=10:
    assert_eq!(sum_of_squares(10), 385);
    assert_eq!(square(sum_to(10)), 3025);
    assert_eq!(difference(10), 3025 - 385);
    let _ = d;
}
