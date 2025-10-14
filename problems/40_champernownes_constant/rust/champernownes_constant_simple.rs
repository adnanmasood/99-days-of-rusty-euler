// champernownes_constant_simple.rs
// Project Euler 40 - Champernowne's Constant (simple version)
//
// Build the concatenated string "123456789101112..." until we reach at least
// 1_000_000 digits in the fractional part. Then pick out d_1, d_10, ...
// and multiply them. This is intentionally straightforward and readable.

fn main() {
    // Build the digit tape up to the largest index we need.
    const NEED: usize = 1_000_000;
    let mut s = String::with_capacity(NEED + 20);
    let mut n: usize = 1;
    while s.len() < NEED {
        s.push_str(&n.to_string());
        n += 1;
    }

    // Helper to fetch the nth digit (1-based) from the fractional part.
    let get = |idx: usize| -> u32 {
        s.as_bytes()[idx - 1] as u32 - b'0' as u32
    };

    let targets = [1usize, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let mut prod: u64 = 1;
    for t in targets {
        prod *= get(t) as u64;
    }

    // println!("{}", prod); // TODO: reveal locally
    println!("product d1*d10*...*d1000000 = <redacted>");

    // Quick sanity: the statement says d_12 = 1
    assert_eq!(get(12), 1);
}
