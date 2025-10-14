// reciprocal_cycles.go
// Go reference solution for Project Euler 26. Final answer redacted.
package main

import "fmt"

func stripTwosFives(n int) int {
    for n%2 == 0 { n /= 2 }
    for n%5 == 0 { n /= 5 }
    return n
}

func cycleLen(d int) int {
    n := stripTwosFives(d)
    if n == 1 { return 0 }
    k := 1
    rem := 10 % n
    for rem != 1 {
        rem = (rem * 10) % n
        k++
    }
    return k
}

func main() {
    if cycleLen(2) != 0 || cycleLen(3) != 1 || cycleLen(7) != 6 {
        panic("sanity failed")
    }
    bestD, bestLen := 0, 0
    for d := 2; d < 1000; d++ {
        if l := cycleLen(d); l > bestLen {
            bestLen = l
            bestD = d
        }
    }
    // fmt.Println(bestD, bestLen) // TODO: reveal locally
    fmt.Println("d<1000 with longest recurring cycle: d=[redacted], len=[redacted]")
    _, _ = bestD, bestLen
}
