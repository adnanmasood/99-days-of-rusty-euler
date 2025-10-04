// special_pythagorean_triplet_simple.rs
// Brute-force search for a<b<c with a+b+c = S and a^2 + b^2 = c^2.
// Final print redacted per near-finish policy.

const S: i32 = 1000;

fn main() {
    let mut product: Option<i64> = None;
    let mut triple: Option<(i32, i32, i32)> = None;

    // a < b < c and a + b + c = S
    for a in 1..S {
        for b in (a + 1)..S {
            let c = S - a - b;
            if c <= b {
                break;
            }
            if a * a + b * b == c * c {
                product = Some(a as i64 * b as i64 * c as i64);
                triple = Some((a, b, c));
                break;
            }
        }
        if product.is_some() {
            break;
        }
    }

    if let Some((a, b, c)) = triple {
        let _product = product.expect("product computed");
        // println!("Found: a={}, b={}, c={}, product={}", a, b, c, _product);
        println!("Found Pythagorean triplet with sum {}. Product = <compute locally>", S);
    }

    // Tiny sanity: (3,4,5) works and sums to 12
    assert!(3 * 3 + 4 * 4 == 5 * 5);
}
