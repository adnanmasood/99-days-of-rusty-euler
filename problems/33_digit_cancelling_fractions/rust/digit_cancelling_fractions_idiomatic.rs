// digit_cancelling_fractions_idiomatic.rs
// Project Euler 33 - Digit Cancelling Fractions (idiomatic & robust Rust)
//
// This version does digit work arithmetically (no strings), isolates the
// "curious fraction" predicate, and reduces the final product by gcd.

#[inline]
fn digits2(x: u32) -> (u32, u32) { (x / 10, x % 10) }

#[inline]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[inline]
fn is_curious(n: u32, d: u32) -> bool {
    // Exclude zeros to avoid trivial examples like 30/50.
    let (nt, nu) = digits2(n);
    let (dt, du) = digits2(d);
    if nu == 0 || du == 0 || nt == 0 || dt == 0 { return false; }

    // Try cancelling any common digit that appears exactly once in each.
    // We consider four placements; we ensure the other digit on each side forms a 1-digit number.
    let mut check = |keep_n: u32, keep_d: u32| -> bool {
        keep_d != 0 && (n as u64) * (keep_d as u64) == (d as u64) * (keep_n as u64)
    };

    // common = nu == dt -> compare nt / du
    if nu == dt && check(nt, du) { return true; }
    // common = nt == du -> compare nu / dt
    if nt == du && check(nu, dt) { return true; }
    // common = nt == dt -> compare nu / du  (rare; generally false but safe to test)
    if nt == dt && check(nu, du) { return true; }
    // common = nu == du -> compare nt / dt
    if nu == du && check(nt, dt) { return true; }

    false
}

pub fn solve() -> u64 {
    let mut num: u64 = 1;
    let mut den: u64 = 1;
    for n in 10..=99 {
        for d in (n+1)..=99 {
            if is_curious(n, d) {
                num *= n as u64;
                den *= d as u64;
            }
        }
    }
    let g = gcd(num, den);
    den / g
}

fn main() {
    let ans = solve();
    // println!("{}", ans); // TODO: reveal locally
    println!("denominator (lowest terms) = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_true() {
        assert!(is_curious(49, 98));
    }
    #[test]
    fn zero_rejected() {
        assert!(!is_curious(30, 50));
    }
}
