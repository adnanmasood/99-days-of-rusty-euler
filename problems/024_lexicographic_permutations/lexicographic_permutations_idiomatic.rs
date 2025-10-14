// lexicographic_permutations_idiomatic.rs
// A compact, idiomatic implementation using a precomputed factorial table and factoradic digits.
// Final numeric answer is redacted per series rules.

fn build_factorials(n: usize) -> Vec<u64> {
    let mut fact = vec![1u64; n + 1];
    for i in 1..=n { fact[i] = fact[i-1] * (i as u64); }
    fact
}

/// Returns the n-th (1-based) permutation of 'items' in lexicographic order.
fn nth_perm(mut items: Vec<char>, n1: u64) -> String {
    let m = items.len();
    let fact = build_factorials(m);
    let mut k = n1 - 1; // zero-based
    let mut out = String::with_capacity(m);
    for r in (1..=m).rev() {
        let f = fact[r - 1];
        let idx = (k / f) as usize;
        k %= f;
        out.push(items.remove(idx));
    }
    out
}

fn main() {
    // Unit sanity: {0,1,2} -> 3rd is "102"
    assert_eq!(nth_perm(vec!['0','1','2'], 3), "102");

    let digits: Vec<char> = ('0'..='9').collect();
    let millionth = nth_perm(digits, 1_000_000);
    // println!("{}", millionth); // TODO: reveal locally
    println!("Millionth permutation of 0..9 = [redacted]");
    let _ = millionth;
}
