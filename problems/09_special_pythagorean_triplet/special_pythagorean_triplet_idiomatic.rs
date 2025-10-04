// special_pythagorean_triplet_idiomatic.rs
// Euclid's formula + arithmetic constraints; final value redacted.

const S: i64 = 1000;

#[inline]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

/// Find (a,b,c) with a<b<c, a^2+b^2=c^2 and a+b+c=S using Euclid's parameterization:
/// a = k*(m^2 - n^2), b = k*(2mn), c = k*(m^2 + n^2), S = 2*k*m*(m+n).
pub fn find_triplet_sum(s: i64) -> Option<(i64, i64, i64)> {
    // m > n >= 1, gcd(m,n)=1, (m-n) odd
    let mut m: i64 = 2;
    while 2 * m * (m + 1) <= s {
        let mut n: i64 = 1;
        while n < m {
            if ((m - n) & 1) == 1 && gcd(m, n) == 1 {
                let denom = 2 * m * (m + n);
                if s % denom == 0 {
                    let k = s / denom;
                    let a = k * (m * m - n * n);
                    let b = k * (2 * m * n);
                    let c = k * (m * m + n * n);
                    let (a, b) = if a < b { (a, b) } else { (b, a) };
                    return Some((a, b, c));
                }
            }
            n += 1;
        }
        m += 1;
    }
    None
}

fn main() {
    if let Some((a, b, c)) = find_triplet_sum(S) {
        let _prod = a * b * c;
        // println!("a={}, b={}, c={}, product={}", a, b, c, _prod); // TODO: reveal locally
        println!("(a,b,c) found. Product = <compute locally>");
    }
    // Example: (3,4,5) when S=12
    if let Some((aa, bb, cc)) = find_triplet_sum(12) {
        assert_eq!(aa * aa + bb * bb, cc * cc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_4_5() {
        let (a, b, c) = find_triplet_sum(12).expect("triplet for 12");
        assert_eq!(a * a + b * b, c * c);
        assert!(a < b && b < c);
    }
}
