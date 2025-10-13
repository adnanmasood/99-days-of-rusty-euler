// digit_fifth_powers_idiomatic.rs
// Project Euler 30 — Idiomatic Rust with iterators and small helpers.
// Final Euler answer intentionally not printed.

fn pow10_table(power: u32) -> [u32; 10] {
    let mut t = [0u32; 10];
    let mut acc;
    for d in 0..=9u32 {
        acc = 1u32;
        for _ in 0..power { acc *= d; }
        t[d as usize] = acc;
    }
    t
}

fn digit_power_sum(mut n: u32, table: &[u32; 10]) -> u32 {
    if n == 0 { return table[0]; }
    let mut s = 0u32;
    while n > 0 {
        s += table[(n % 10) as usize];
        n /= 10;
    }
    s
}

fn search(power: u32) -> u32 {
    let table = pow10_table(power);
    let nine_pow = table[9];
    let upper = 6 * nine_pow; // argument as in the simple version

    (2..=upper)
        .filter(|&n| digit_power_sum(n, &table) == n)
        .sum()
}

fn main() {
    assert_eq!(search(4), 19316); // given example for fourth powers
    let ans = search(5);
    // println!("{}", ans); // TODO: reveal locally
    println!("Sum for power 5: [redacted]");
    let _ = ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fourth_power_sum() {
        assert_eq!(search(4), 19316);
    }
}
