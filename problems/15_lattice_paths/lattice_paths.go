// lattice_paths.go
// Project Euler 15 - Lattice Paths (Go)
// Use math/big to compute C(2n, n) generically; print placeholder.
package main

import (
    "fmt"
    "math/big"
)

func binom(n, k int64) *big.Int {
    if k < 0 || k > n {
        return big.NewInt(0)
    }
    if k == 0 || k == n {
        return big.NewInt(1)
    }
    if k > n-k {
        k = n - k
    }
    num := big.NewInt(1)
    den := big.NewInt(1)
    for i := int64(1); i <= k; i++ {
        num.Mul(num, big.NewInt(n-k+i))
        den.Mul(den, big.NewInt(i))
    }
    return new(big.Int).Div(num, den)
}

func main() {
    // check 2x2 -> 6
    if binom(4, 2).Cmp(big.NewInt(6)) != 0 {
        panic("sanity check failed")
    }
    ans := binom(40, 20)
    _ = ans
    // fmt.Println(ans.String()) // TODO: reveal locally
    fmt.Println("Computed C(40,20) with big.Int (redacted).")
}
