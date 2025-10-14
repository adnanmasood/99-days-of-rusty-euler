// thousand_digit_fibonacci_number_simple.rs
// Project Euler 25 — 1000-digit Fibonacci Number (simple version)
// We avoid external crates by adding big integers as decimal strings.
// Final Euler answer intentionally not printed.

fn add_dec_str(a: &str, b: &str) -> String {
    // Add two non-empty decimal strings and return their sum as a string.
    let bytes_a = a.as_bytes();
    let bytes_b = b.as_bytes();
    let mut i: isize = bytes_a.len() as isize - 1;
    let mut j: isize = bytes_b.len() as isize - 1;
    let mut carry: i32 = 0;
    let mut out: Vec<u8> = Vec::with_capacity(a.len().max(b.len()) + 1);

    while i >= 0 || j >= 0 || carry > 0 {
        let da = if i >= 0 { (bytes_a[i as usize] - b'0') as i32 } else { 0 };
        let db = if j >= 0 { (bytes_b[j as usize] - b'0') as i32 } else { 0 };
        let s = da + db + carry;
        out.push((b'0' + (s % 10) as u8) as u8);
        carry = s / 10;
        i -= 1;
        j -= 1;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn first_fib_index_with_digits(target_digits: usize) -> usize {
    // F1 = "1", F2 = "1"
    let mut a = String::from("1");
    let mut b = String::from("1");
    if target_digits <= 1 { return 1; }
    let mut index: usize = 2;
    loop {
        let c = add_dec_str(&a, &b);
        index += 1;
        if c.len() >= target_digits {
            return index;
        }
        a = b;
        b = c;
    }
}

fn main() {
    // Sanity: given in the statement, F12 = 144 has 3 digits.
    let idx_3 = first_fib_index_with_digits(3);
    debug_assert_eq!(idx_3, 12);

    let idx_1000 = first_fib_index_with_digits(1000);
    // println!("{}", idx_1000); // TODO: reveal locally
    println!("Index of first Fibonacci term with 1000 digits = [redacted]");
    let _ = idx_1000; // avoid unused warnings
}
