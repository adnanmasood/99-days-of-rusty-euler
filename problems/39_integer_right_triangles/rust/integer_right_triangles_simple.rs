// integer_right_triangles_simple.rs
// Project Euler 39 - Integer Right Triangles (simple version)
//
// Brute-force perimeter scan up to 1000. For each perimeter p, we count the
// integer right triangles (a,b,c) with a^2 + b^2 = c^2 and a + b + c = p.
// We use the ordering a < b < c to avoid duplicates.
//
// This solution favors clarity over speed; it's still plenty fast for p <= 1000.
//
// No-spoilers policy: we compute the maximizing perimeter but print a placeholder.

fn count_solutions(p: u32) -> u32 {
    let mut cnt = 0u32;
    // a < b < c, and a + b + c = p
    // a must be < p/3; b must be < (p - a)/2
    for a in 1..=p/3 {
        for b in (a+1)..=((p - a)/2) {
            let c = p - a - b;
            if a*a + b*b == c*c {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let mut best_p = 0u32;
    let mut best_cnt = 0u32;
    for p in 3..=1000 {
        let c = count_solutions(p);
        if c > best_cnt {
            best_cnt = c;
            best_p = p;
        }
    }
    println!("maximizing perimeter (p <= 1000) = <redacted>");
    // println!("p = {}, solutions = {}", best_p, best_cnt); // TODO: reveal locally
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p120_has_three() {
        assert_eq!(count_solutions(120), 3);
    }
}
