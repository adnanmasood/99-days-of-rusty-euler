// largest_palindrome_product_simple.rs
// Straightforward nested loops with a numeric palindrome test.
// Final answer redacted per the near-finish policy.

fn is_palindrome(mut n: u32) -> bool {
    let original = n;
    let mut rev = 0u32;
    while n > 0 {
        rev = rev * 10 + (n % 10);
        n /= 10;
    }
    original == rev
}

/// Brute-force search (with small de-dup optimization j >= i).
fn largest_palindrome_product_3digits_simple() -> u32 {
    let mut best: u32 = 0;
    for i in 100u32..=999 {
        for j in i..=999 {
            let prod = i * j;
            if prod > best && is_palindrome(prod) {
                best = prod;
            }
        }
    }
    best
}

fn main() {
    let best = largest_palindrome_product_3digits_simple();
    // println!("{best}"); // TODO: reveal locally
    println!("largest pal product (3 digits) = <compute locally>");

    // Statement sanity check for 2-digit case: 9009 = 91 * 99.
    let mut check_best = 0;
    for a in 10u32..=99 {
        for b in a..=99 {
            let p = a * b;
            if p > check_best && is_palindrome(p) {
                check_best = p;
            }
        }
    }
    assert_eq!(check_best, 9009);
    let _ = best; // avoid unused warning when print is redacted
}
