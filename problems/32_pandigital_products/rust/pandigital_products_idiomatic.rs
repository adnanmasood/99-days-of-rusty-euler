// pandigital_products_idiomatic.rs
// Project Euler 32 - Pandigital Products (idiomatic & robust Rust)
//
// This version avoids string allocation by using digit bitmasks,
// prunes by digit-length shapes (1x4 and 2x3), and isolates pure helpers.
// We still deduplicate products via HashSet and keep a no-spoiler print.

use std::collections::HashSet;

const TARGET_MASK: u16 = 0b11_1111_1110; // bits 1..9 set; bit 0 is unused

#[inline]
fn digit_mask_no_zero_unique(mut n: u32) -> Option<u16> {
    // Return the bitmask of digits if n has no zeros and no repeated digits; otherwise None.
    if n == 0 { return None; }
    let mut mask: u16 = 0;
    while n > 0 {
        let d = (n % 10) as u16;
        if d == 0 { return None; }
        let bit = 1u16 << d;
        if (mask & bit) != 0 { return None; } // repeated digit within this number
        mask |= bit;
        n /= 10;
    }
    Some(mask)
}

#[inline]
fn is_pandigital_identity(a: u32, b: u32) -> bool {
    let p = a * b;

    let (Some(ma), Some(mb), Some(mp)) = (digit_mask_no_zero_unique(a), digit_mask_no_zero_unique(b), digit_mask_no_zero_unique(p)) else {
        return false;
    };

    // Disjoint digits across a, b, and p
    if (ma & mb) != 0 || (ma & mp) != 0 || (mb & mp) != 0 {
        return false;
    }

    // All digits 1..9 must appear exactly once in total
    let union = ma | mb | mp;
    if union != TARGET_MASK {
        return false;
    }

    // Ensure the concatenated length is exactly 9 digits
    let digits_total = ma.count_ones() + mb.count_ones() + mp.count_ones();
    digits_total == 9
}

pub fn solve() -> u32 {
    let mut products = HashSet::<u32>::new();

    // 1-digit × 4-digit
    for a in 1..=9u32 {
        for b in 1000..=9999u32 {
            if is_pandigital_identity(a, b) {
                products.insert(a * b);
            }
        }
    }

    // 2-digit × 3-digit
    for a in 10..=99u32 {
        for b in 100..=999u32 {
            if is_pandigital_identity(a, b) {
                products.insert(a * b);
            }
        }
    }

    products.iter().copied().sum::<u32>()
}

fn main() {
    let total = solve();
    // println!("{}", total); // TODO: reveal locally
    println!("pandigital products sum = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_identity() {
        assert!(is_pandigital_identity(39, 186));
    }

    #[test]
    fn masks_work() {
        assert_eq!(digit_mask_no_zero_unique(123).unwrap().count_ones(), 3);
        assert!(digit_mask_no_zero_unique(101).is_none()); // has a zero and repeat '1'
    }
}
