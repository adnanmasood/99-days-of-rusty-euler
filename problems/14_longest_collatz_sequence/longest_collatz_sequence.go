// longest_collatz_sequence.go
// Project Euler 14 - Longest Collatz Sequence (Go)
// Memoized solution with maps and slices; prints placeholder.
package main

import "fmt"

func nextCollatz(n uint64) uint64 {
    if n%2 == 0 { return n / 2 }
    return 3*n + 1
}

func main() {
    const LIMIT = 1000000
    small := make([]uint32, LIMIT+1)
    small[1] = 1
    big := make(map[uint64]uint32)
    var bestStart uint64 = 1
    var bestLen uint32 = 1

    for s := uint64(1); s < LIMIT; s++ {
        x := s
        trail := make([]uint64, 0, 200)
        var known uint32
        for {
            if x == 1 {
                known = 1; break
            }
            if x <= LIMIT && small[x] != 0 {
                known = small[x]; break
            }
            if v, ok := big[x]; ok {
                known = v; break
            }
            trail = append(trail, x)
            x = nextCollatz(x)
        }
        l := known
        for i := len(trail)-1; i >= 0; i-- {
            l++
            y := trail[i]
            if y <= LIMIT { small[y] = l } else { big[y] = l }
        }
        if l > bestLen { bestLen, bestStart = l, s }
    }

    // fmt.Println(bestStart, bestLen) // TODO
    fmt.Println("Computed longest chain under 1e6 (redacted).")
}
