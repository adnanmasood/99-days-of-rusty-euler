// largest_palindrome_product_idiomatic.rs
// Faster search using properties of even-length palindromes and pruning.
//
// Facts we use:
// 1) Any 6-digit palindrome is divisible by 11.
// 2) When scanning i from high to low, we can restrict j to maintain that
//    at least one of (i, j) is a multiple of 11. That lets us step j by 11
//    whenever i is not a multiple of 11.
// 3) Early exit: if i * j_max <= best (where j_max is the largest valid j for this i),
//    nothing in this row can beat `best`.

#[inline]
fn is_palindrome(mut n: u32) -> bool {
    let original = n;
    let mut rev = 0u32;
    while n > 0 {
        rev = rev * 10 + (n % 10);
        n /= 10;
    }
    original == rev
}

pub fn largest_palindrome_product_3digits() -> u32 {
    let mut best: u32 = 0;
    // Descend so pruning becomes effective quickly.
    for i in (100u32..=999).rev() {
        // Ensure one multiplicand is a multiple of 11.
        let step = if i % 11 == 0 { 1 } else { 11 };
        let mut j = if step == 11 { 990 } else { 999 }; // largest <= 999 with proper step

        // If even the largest product with this i cannot beat `best`, skip the row.
        if i * j <= best {
            continue;
        }

        while j >= i {
            let prod = i * j;
            if prod <= best {
                break; // nothing further in this row can beat best
            }
            if is_palindrome(prod) {
                best = prod;
            }
            j -= step;
        }
    }
    best
}

// Alternative strategy: generate palindromes high -> low and factor-check.
pub fn largest_palindrome_product_by_generation() -> u32 {
    // A 6-digit palindrome has the form abccba = 100001a + 10010b + 1100c.
    for a in (1u32..=9).rev() {
        for b in (0u32..=9).rev() {
            for c in (0u32..=9).rev() {
                let pal = 100_001u32 * a + 10_010u32 * b + 1_100u32 * c;
                for d in (100u32..=999).rev() {
                    if d % 11 != 0 {
                        continue;
                    }
                    if pal % d == 0 {
                        let q = pal / d;
                        if (100u32..=999).contains(&q) {
                            return pal; // first found is the largest because we count down
                        }
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let best = largest_palindrome_product_3digits();
    let best2 = largest_palindrome_product_by_generation();
    // println!("best via search = {best}, best via generation = {best2}"); // TODO: reveal locally
    println!("best via search, best via generation = <compute locally>");
    let _ = (best, best2);

    // 2-digit example from the statement for sanity.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_digit_example() {
        // Largest palindrome from two 2-digit numbers is 9009.
        let mut best = 0;
        for a in 10u32..=99 {
            for b in a..=99 {
                let p = a * b;
                if p > best && is_palindrome(p) {
                    best = p;
                }
            }
        }
        assert_eq!(best, 9009);
    }
}
