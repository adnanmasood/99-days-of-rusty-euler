// lexicographic_permutations.go
// Go reference solution using factoradic indexing. Final answer redacted.
package main

import (
    "fmt"
)

func buildFact(n int) []uint64 {
    f := make([]uint64, n+1)
    f[0] = 1
    for i := 1; i <= n; i++ {
        f[i] = f[i-1] * uint64(i)
    }
    return f
}

func nthPerm(items []rune, n1 uint64) string {
    m := len(items)
    fact := buildFact(m)
    k := n1 - 1 // zero-based
    out := make([]rune, 0, m)
    for r := m; r >= 1; r-- {
        f := fact[r-1]
        idx := int(k / f)
        k = k % f
        out = append(out, items[idx])
        items = append(items[:idx], items[idx+1:]...)
    }
    return string(out)
}

func main() {
    // sanity
    if nthPerm([]rune{'0','1','2'}, 3) != "102" {
        panic("sanity check failed")
    }
    digits := []rune{'0','1','2','3','4','5','6','7','8','9'}
    millionth := nthPerm(digits, 1_000_000)
    // fmt.Println(millionth) // TODO: reveal locally
    fmt.Println("Millionth permutation of 0..9 = [redacted]")
    _ = millionth
}
