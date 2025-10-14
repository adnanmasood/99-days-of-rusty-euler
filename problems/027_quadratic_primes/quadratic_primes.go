// quadratic_primes.go
// Go reference solution for Project Euler 27. Final answer redacted.
package main

import (
    "fmt"
    "math"
)

func isPrime(n int64) bool {
    if n <= 1 { return false }
    if n % 2 == 0 { return n == 2 }
    var d int64 = 3
    for d*d <= n {
        if n % d == 0 { return false }
        d += 2
    }
    return true
}

func runLen(a, b int64) int64 {
    var n int64 = 0
    for {
        val := n*n + a*n + b
        if !isPrime(val) { break }
        n++
    }
    return n
}

func main() {
    if runLen(1, 41) != 40 || runLen(-79, 1601) != 80 {
        panic("sanity failed")
    }

    var bestA, bestB, bestLen int64 = 0, 0, 0
    for a := int64(-999); a <= 999; a++ {
        for b := int64(2); b <= 1000; b++ {
            if !isPrime(b) { continue }
            l := runLen(a, b)
            if l > bestLen {
                bestLen, bestA, bestB = l, a, b
            }
        }
    }

    product := bestA * bestB
    // fmt.Println(product) // TODO: reveal locally
    fmt.Println("Best (a,b,len,product) = [redacted]")
    _ = product
}
