// digit_cancelling_fractions_simple.rs
// Project Euler 33 - Digit Cancelling Fractions (simple, didactic version)
//
// We search all two-digit fractions n/d < 1 (10<=n<d<=99) where erroneously
// "cancelling" a shared non-zero digit yields an equal fraction.
// We exclude trivial examples that involve a 0 digit (e.g., 30/50).
// There are exactly four such non-trivial examples; we multiply them together,
// reduce to lowest terms, and (normally) output the denominator.
// Here we print a placeholder to keep the series spoiler-free.

fn is_curious(n: u32, d: u32) -> bool {
    // reject trivial zeros
    if n % 10 == 0 || d % 10 == 0 { return false; }

    let ns = n.to_string();
    let ds = d.to_string();

    // Find a common non-zero digit and try cancelling it.
    for ch in b'1'..=b'9' {
        let c = ch as char;
        let n_count = ns.matches(c).count();
        let d_count = ds.matches(c).count();
        if n_count == 1 && d_count == 1 {
            // Remove one occurrence of c from each
            let mut ns2 = String::new();
            let mut removed_n = false;
            for ch2 in ns.chars() {
                if ch2 == c && !removed_n { removed_n = true; } else { ns2.push(ch2); }
            }
            let mut ds2 = String::new();
            let mut removed_d = false;
            for ch2 in ds.chars() {
                if ch2 == c && !removed_d { removed_d = true; } else { ds2.push(ch2); }
            }
            if ns2.len() == 1 && ds2.len() == 1 {
                let n2: u32 = ns2.parse().unwrap();
                let d2: u32 = ds2.parse().unwrap();
                if d2 != 0 && n * d2 == d * n2 {
                    return true;
                }
            }
        }
    }
    false
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    let mut num_prod: u64 = 1;
    let mut den_prod: u64 = 1;

    for n in 10u32..=99u32 {
        for d in (n+1)..=99u32 {
            if is_curious(n, d) {
                num_prod *= n as u64;
                den_prod *= d as u64;
            }
        }
    }

    let g = gcd(num_prod, den_prod);
    let denom_in_lowest = den_prod / g;

    // println!("{}", denom_in_lowest); // TODO: reveal locally
    println!("denominator (lowest terms) = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_example_works() {
        // From the statement: 49/98 -> cancel '9' to get 4/8; still equal.
        assert!(is_curious(49, 98));
    }
    #[test]
    fn trivial_zero_is_rejected() {
        assert!(!is_curious(30, 50));
    }
}
