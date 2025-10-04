// highly_divisible_triangular_number_simple.rs
// Project Euler 12 - Highly Divisible Triangular Number
// Straightforward search using trial division to count divisors of each triangle number.
// We avoid printing the final numeric answer to respect the no-spoiler policy.
fn count_divisors(mut n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut count = 1u64;
    let mut exp = 0u64;
    // Factor out multiples of 2
    while n % 2 == 0 {
        n /= 2;
        exp += 1;
    }
    if exp > 0 { count *= exp + 1; }
    // Factor odd components up to sqrt(n)
    let mut p = 3u64;
    while p * p <= n {
        exp = 0;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        if exp > 0 { count *= exp + 1; }
        p += 2;
    }
    // Whatever remains is prime
    if n > 1 { count *= 2; }
    count
}
fn main() {
    let target: u64 = 500; // look for the first triangle number with > 500 divisors
    let mut n: u64 = 1;
    let mut tri: u64 = 1; // T_1
    loop {
        let d = count_divisors(tri);
        if d > target {
            // println!("First triangle number with > {} divisors is {}", target, tri); // TODO
            println!("Found the target triangle number. (Redacted to avoid spoilers.)");
            break;
        }
        n += 1;
        tri += n; // next triangular number
    }
}
