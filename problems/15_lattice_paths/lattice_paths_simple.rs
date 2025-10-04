// lattice_paths_simple.rs
// Project Euler 15 - Lattice Paths
// Dynamic programming on an (n x n) grid; moves only Right or Down.
// We avoid printing the final Euler answer per the no-spoiler policy.

fn lattice_paths_dp(n: usize) -> u128 {
    // dp[r][c] = number of ways to reach (r,c) from (0,0) moving only Right/Down.
    let mut dp = vec![vec![0u128; n + 1]; n + 1];
    // First row and first column each have exactly one way (all rights or all downs).
    for i in 0..=n {
        dp[0][i] = 1;
        dp[i][0] = 1;
    }
    for r in 1..=n {
        for c in 1..=n {
            dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
        }
    }
    dp[n][n]
}

fn main() {
    // Tiny sanity check from the statement: 2x2 grid -> 6 routes.
    let small = lattice_paths_dp(2);
    assert_eq!(small, 6);

    let n = 20usize;
    let ways = lattice_paths_dp(n);
    let _ = ways; // computed but redacted
    // println!("Number of lattice paths in a {}x{} grid: {}", n, n, ways); // TODO
    println!("Computed number of lattice paths for 20x20. (Redacted.)");
}
