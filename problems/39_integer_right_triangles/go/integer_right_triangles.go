// integer_right_triangles.go
// Project Euler 39 - Integer Right Triangles (Go)
package main

import "fmt"

func gcd(a, b int) int {
    for b != 0 {
        a, b = b, a%b
    }
    return a
}

func main() {
    const LIMIT = 1000
    counts := make([]int, LIMIT+1)
    for m := 2; 2*m*(m+1) <= LIMIT; m++ {
        for n := 1; n < m; n++ {
            if ((m-n)&1) == 1 && gcd(m, n) == 1 {
                p0 := 2 * m * (m + n)
                for k := 1; k*p0 <= LIMIT; k++ {
                    counts[k*p0]++
                }
            }
        }
    }
    bestP, bestC := 0, 0
    for p := 0; p <= LIMIT; p++ {
        if counts[p] > bestC {
            bestC = counts[p]
            bestP = p
        }
    }
    // fmt.Printf("p = %d, solutions = %d\n", bestP, bestC) // TODO: reveal locally
    fmt.Println("maximizing perimeter (p <= 1000) = <redacted>")
}
