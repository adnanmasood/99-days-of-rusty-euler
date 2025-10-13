// thousand_digit_fibonacci_number.go
// Project Euler 25 — Go solution using big.Int. Final answer redacted.
package main

import (
    "fmt"
    "math/big"
)

func firstIndexWithDigits(d int) int {
    if d <= 1 { return 1 }
    a := big.NewInt(1)
    b := big.NewInt(1)
    idx := 2
    tmp := new(big.Int)
    for {
        tmp.Add(a, b) // tmp = a + b
        idx++
        if len(tmp.String()) >= d {
            return idx
        }
        a.Set(b)
        b.Set(tmp)
        tmp = new(big.Int) // allocate a fresh big.Int for next iteration
    }
}

func main() {
    if firstIndexWithDigits(3) != 12 {
        panic("sanity failed")
    }
    idx := firstIndexWithDigits(1000)
    // fmt.Println(idx) // TODO: reveal locally
    fmt.Println("Index of first Fibonacci term with 1000 digits = [redacted]")
    _ = idx
}
