// double_base_palindromes_simple.rs
// Project Euler 36 - Double-base Palindromes (simple, didactic version)
//
// Sum all numbers below one million that are palindromic in base 10 and base 2.
// Representations may not include leading zeros.
// This simple build scans odd numbers < 1_000_000 (evens can't be binary palindromes)
// and checks palindromicity in base 10 and base 2 using a shared helper.
//
// No spoilers: we print a placeholder instead of the actual sum.

const LIMIT: u32 = 1_000_000;

fn is_palindrome_base(mut n: u32, base: u32) -> bool {
    // Collect digits (least significant first), then check mirror equality.
    if n == 0 { return true; }
    let mut digits: Vec<u32> = Vec::new();
    while n > 0 {
        digits.push(n % base);
        n /= base;
    }
    let mut i = 0usize;
    let mut j = digits.len().saturating_sub(1);
    while i < j {
        if digits[i] != digits[j] { return false; }
        i += 1;
        j -= 1;
    }
    true
}

fn main() {
    let mut total: u64 = 0;
    // 0 is not considered; skip evens since binary palindromes can't end with 0 unless the number is 0.
    for n in (1u32..LIMIT).step_by(2) {
        if is_palindrome_base(n, 10) && is_palindrome_base(n, 2) {
            total += n as u64;
        }
    }
    // println!("{}", total); // TODO: reveal locally
    println!("sum of double-base palindromes below one million = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_585() {
        // From the statement: 585 is palindromic in base 10 and base 2 (1001001001)
        assert!(is_palindrome_base(585, 10));
        assert!(is_palindrome_base(585, 2));
    }
}
