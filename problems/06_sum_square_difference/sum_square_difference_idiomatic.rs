// sum_square_difference_idiomatic.rs
// Algebraic simplification and helpers; final print redacted.

#[inline]
fn sum_to(n: u128) -> u128 {
    n * (n + 1) / 2
}

#[inline]
fn sum_of_squares(n: u128) -> u128 {
    n * (n + 1) * (2 * n + 1) / 6
}

#[inline]
fn square(x: u128) -> u128 {
    x * x
}

/// difference(n) = (sum_{k=1}^n k)^2 - sum_{k=1}^n k^2
pub fn difference(n: u128) -> u128 {
    square(sum_to(n)) - sum_of_squares(n)
}

fn main() {
    const N: u128 = 100;
    let ans = difference(N);
    // println!("Answer: {}", ans); // TODO: reveal locally
    println!("D(n) = (n(n+1)/2)^2 - n(n+1)(2n+1)/6  at n=100 -> <compute locally>");
    let _ = ans;
    // Example from statement:
    assert_eq!(difference(10), 3025 - 385);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny() {
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_of_squares(1), 1);
        assert_eq!(difference(1), 0);
    }

    #[test]
    fn example_10() {
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_of_squares(10), 385);
        assert_eq!(difference(10), 3025 - 385);
    }
}
