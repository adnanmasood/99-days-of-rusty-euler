// number_spiral_diagonals_simple.rs
// Project Euler 28 — Number Spiral Diagonals (simple layered walk).
// We'll accumulate the diagonal numbers by noticing that in the layer
// of side length s (odd), the four corners differ by step = s - 1.
// Final Euler answer intentionally not printed.

fn sum_spiral_diagonals_simple(n: u64) -> u128 {
    assert!(n % 2 == 1, "n must be odd (spiral center is 1).");
    if n == 1 { return 1; }

    let mut total: u128 = 1;
    let mut current: u128 = 1;

    // Grow the spiral in layers: side length 3, 5, 7, ..., n
    let mut side: u64 = 3;
    while side <= n {
        let step = (side - 1) as u128;
        // add four corners of this layer
        for _ in 0..4 {
            current += step;
            total += current;
        }
        side += 2;
    }
    total
}

fn main() {
    // Sanity checks from the statement: 1x1 -> 1, 3x3 -> 25, 5x5 -> 101.
    assert_eq!(sum_spiral_diagonals_simple(1), 1);
    assert_eq!(sum_spiral_diagonals_simple(3), 25);
    assert_eq!(sum_spiral_diagonals_simple(5), 101);

    let n = 1001;
    let answer = sum_spiral_diagonals_simple(n);
    // println!("{}", answer); // TODO: reveal locally
    println!("Sum of diagonals for {}x{} spiral = [redacted]", n, n);
    let _ = answer;
}
