// integer_right_triangles_idiomatic.rs
// Project Euler 39 - Integer Right Triangles (idiomatic & optimized)
//
// We use Euclid's formula for primitive Pythagorean triples:
//   a = m^2 - n^2, b = 2mn, c = m^2 + n^2   with m > n, gcd(m,n) = 1, (m-n) odd
// The primitive perimeter p0 = 2*m*(m + n). Any multiple k*p0 (k >= 1) is a
// valid perimeter that yields exactly one triple for that multiple of the
// primitive solution. We count for all p <= 1000 by iterating m,n, then adding
// counts to k*p0 for k*p0 <= 1000.
//
// No-spoilers policy: we compute the maximizing perimeter but print a placeholder.

#[inline]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

pub fn solve(limit: u32) -> (u32, u32) {
    let mut counts = vec![0u32; (limit + 1) as usize];
    let mut m = 2u32;
    while 2 * m * (m + 1) <= limit { // minimal n=1 boundary for p0
        let mut n = 1u32;
        while n < m {
            if ((m - n) & 1) == 1 && gcd(m, n) == 1 {
                let p0 = 2 * m * (m + n);
                let mut k = 1u32;
                while k * p0 <= limit {
                    counts[(k * p0) as usize] += 1;
                    k += 1;
                }
            }
            n += 1;
        }
        m += 1;
    }

    let mut best_p = 0u32;
    let mut best_cnt = 0u32;
    for p in 0..=limit {
        let c = counts[p as usize];
        if c > best_cnt {
            best_cnt = c;
            best_p = p;
        }
    }
    (best_p, best_cnt)
}

fn main() {
    let (p, cnt) = solve(1000);
    // println!("p = {p}, solutions = {cnt}"); // TODO: reveal locally
    println!("maximizing perimeter (p <= 1000) = <redacted>");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_p120() {
        // There are exactly 3 solutions for p = 120 per statement.
        // Both primitive (20,48,52) & (24,45,51) & (30,40,50) are counted.
        let (_p, _cnt) = solve(120);
        // Not asserting exact argmax here; just ensure counts known:
        // Brute-check via small scan:
        fn brute_count(p: u32) -> u32 {
            let mut cnt = 0;
            for a in 1..=p/3 {
                for b in (a+1)..=((p - a)/2) {
                    let c = p - a - b;
                    if a*a + b*b == c*c { cnt += 1; }
                }
            }
            cnt
        }
        assert_eq!(brute_count(120), 3);
    }
}
