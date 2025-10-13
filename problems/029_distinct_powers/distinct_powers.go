// distinct_powers.go
// Go reference solution for Project Euler 29.
// Uses prime-exponent vectors to avoid big integers. Final answer redacted.
package main

import (
    "fmt"
)

func primesUpTo(n int) []int {
    is := make([]bool, n+1)
    for i := range is { is[i] = true }
    is[0], is[1] = false, false
    for p := 2; p*p <= n; p++ {
        if is[p] {
            for m := p*p; m <= n; m += p {
                is[m] = false
            }
        }
    }
    ps := []int{}
    for i := 2; i <= n; i++ {
        if is[i] { ps = append(ps, i) }
    }
    return ps
}

func expsOf(a int, ps []int) []int {
    v := make([]int, len(ps))
    for i, p := range ps {
        if p*p > a { break }
        for a%p == 0 {
            v[i]++
            a /= p
        }
    }
    if a > 1 {
        // find index
        for i, p := range ps {
            if p == a { v[i]++; break }
        }
    }
    return v
}

func distinctCount(aLo, aHi, bLo, bHi int) int {
    ps := primesUpTo(aHi)
    // use map[string]struct{} with a compact string key
    seen := map[string]struct{}{}
    for a := aLo; a <= aHi; a++ {
        base := expsOf(a, ps)
        for b := bLo; b <= bHi; b++ {
            v := make([]int, len(base))
            for i := range base { v[i] = base[i] * b }
            // serialize
            key := make([]byte, 0, len(v)*3)
            for _, e := range v {
                key = append(key, byte(e>>8), byte(e&0xff), ',')
            }
            seen[string(key)] = struct{}{}
        }
    }
    return len(seen)
}

func main() {
    if distinctCount(2,5,2,5) != 15 {
        panic("sanity failed")
    }
    total := distinctCount(2,100,2,100)
    // fmt.Println(total) // TODO: reveal locally
    fmt.Println("Distinct terms for 2<=a<=100, 2<=b<=100: [redacted]")
    _ = total
}
