// lexicographic_permutations_simple.rs
// A clear, loop-first solution for Project Euler 24 (Lexicographic Permutations).
// We compute the nth lexicographic permutation using the factoradic method.
// Final Euler answer intentionally not printed for the 99‑day series.

fn factorial(n: u64) -> u64 {
    (1..=n).product::<u64>()
}

/// Return the n-th (1-based) lexicographic permutation of the given characters.
fn nth_permutation(mut items: Vec<char>, n1: u64) -> String {
    let m = items.len() as u64;
    assert!(m > 0);
    let mut n = n1 - 1; // work in 0-based index for factoradic

    let mut result = String::with_capacity(items.len());

    for rem in (1..=m).rev() {
        let f = factorial(rem - 1);
        let idx = (n / f) as usize;
        n %= f;
        result.push(items.remove(idx));
    }
    result
}

fn main() {
    // Tiny check from the statement: permutations of 0,1,2.
    let demo = nth_permutation(vec!['0','1','2'], 3);
    debug_assert_eq!(demo, "102"); // 3rd permutation is 102

    // Real input:
    let digits: Vec<char> = ('0'..='9').collect();
    let millionth = nth_permutation(digits, 1_000_000);

    // println!("{}", millionth); // TODO: reveal locally
    println!("Millionth permutation of 0..9 = [redacted]");
    let _ = millionth; // avoid unused in release w/o printing
}
