// champernownes_constant_idiomatic.rs
// Project Euler 40 - Champernowne's Constant (idiomatic & O(1) per query)
//
// We avoid building the entire digit tape by using positional arithmetic.
// For a given n (1-based), find its digit group length d, subtract group
// sizes until n sits inside the d-digit block, then compute the exact number
// and which digit inside that number we want.

#[inline]
fn pow10_u32(e: u32) -> u32 {
    let mut r = 1u32;
    for _ in 0..e { r *= 10; }
    r
}

#[inline]
fn digit_at(mut n: u32) -> u32 {
    // d-digit numbers contribute d * 9*10^(d-1) digits.
    let mut d = 1u32;
    let mut block = 9u32 * pow10_u32(d - 1) * d;
    while n > block {
        n -= block;
        d += 1;
        block = 9u32 * pow10_u32(d - 1) * d;
    }
    // Zero-based index within this d-digit block.
    let idx = n - 1;
    let num = pow10_u32(d - 1) + idx / d;
    let digit_index = (idx % d) as usize;
    (num.to_string().as_bytes()[digit_index] - b'0') as u32
}

pub fn solve() -> u64 {
    const TARGETS: [u32; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let mut prod: u64 = 1;
    for &t in &TARGETS {
        prod *= digit_at(t) as u64;
    }
    prod
}

fn main() {
    let _ans = solve();
    // println!("{}", _ans); // TODO: reveal locally
    println!("product d1*d10*...*d1000000 = <redacted>");

    // Sanity from the statement:
    assert_eq!(digit_at(12), 1);
}
