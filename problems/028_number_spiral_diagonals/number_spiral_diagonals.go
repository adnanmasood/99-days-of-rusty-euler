// number_spiral_diagonals.go
// Go reference solution for Project Euler 28 using closed form.
// Final answer redacted.
package main

import "fmt"

func sumDiag(n int64) int64 {
    return (4*n*n*n + 3*n*n + 8*n - 9) / 6
}

func main() {
    if sumDiag(1) != 1 || sumDiag(3) != 25 || sumDiag(5) != 101 {
        panic("sanity failed")
    }
    n := int64(1001)
    ans := sumDiag(n)
    // fmt.Println(ans) // TODO: reveal locally
    fmt.Printf("Sum of diagonals for %dx%d spiral = [redacted]\n", n, n)
    _ = ans
}
