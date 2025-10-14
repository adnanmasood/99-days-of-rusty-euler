// double_base_palindromes_idiomatic.rs
// Project Euler 36 - Double-base Palindromes (idiomatic & robust Rust)
//
// Optimization idea: generate *decimal* palindromes under LIMIT and only
// test those for binary palindromicity. This avoids scanning most numbers.
// We still forbid leading zeros implicitly by how we build palindromes.

const LIMIT: u32 = 1_000_000;

#[inline]
fn is_binary_palindrome(mut n: u32) -> bool {
    if n % 2 == 0 { return false; } // evens end with 0 in binary; leading zeros not allowed
    let mut bits: [u8; 32] = [0; 32];
    let mut len = 0usize;
    while n > 0 {
        bits[len] = (n & 1) as u8;
        len += 1;
        n >>= 1;
    }
    let mut i = 0usize;
    let mut j = len.saturating_sub(1);
    while i < j {
        if bits[i] != bits[j] { return false; }
        i += 1;
        j -= 1;
    }
    true
}

fn make_even_pal(prefix: u32) -> u32 {
    // e.g., 123 -> 123321
    let mut x = prefix;
    let mut r = prefix;
    while x > 0 {
        r = r * 10 + x % 10;
        x /= 10;
    }
    r
}

fn make_odd_pal(prefix: u32, mid: u32) -> u32 {
    // e.g., 123, 4 -> 1234321
    let mut x = prefix;
    let mut r = prefix * 10 + mid;
    while x > 0 {
        r = r * 10 + x % 10;
        x /= 10;
    }
    r
}

pub fn solve() -> u64 {
    let mut total: u64 = 0;

    // 1-digit palindromes
    for n in 1..=9 {
        if is_binary_palindrome(n) { total += n as u64; }
    }

    // Even-length palindromes: 2,4,6 digits -> prefix lengths 1..=3
    for len in 1..=3u32 {
        let start = 10u32.pow(len - 1);
        let end = 10u32.pow(len) - 1;
        for p in start..=end {
            let n = make_even_pal(p);
            if n < LIMIT && is_binary_palindrome(n) {
                total += n as u64;
            }
        }
    }

    // Odd-length palindromes: 3,5 digits -> prefix lengths 1..=2, mid in 0..=9
    for len in 1..=2u32 {
        let start = 10u32.pow(len - 1);
        let end = 10u32.pow(len) - 1;
        for p in start..=end {
            for mid in 0..=9u32 {
                let n = make_odd_pal(p, mid);
                if n < LIMIT && is_binary_palindrome(n) {
                    total += n as u64;
                }
            }
        }
    }

    total
}

fn main() {
    let ans = solve();
    // println!("{}", ans); // TODO: reveal locally
    println!("sum of double-base palindromes below one million = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        // 585 is known to be palindromic in base 10 and base 2.
        assert!(is_binary_palindrome(585));
    }
}
