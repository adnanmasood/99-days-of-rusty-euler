// champernownes_constant.go
// Project Euler 40 - Champernowne's Constant (Go)
package main

import (
    "fmt"
    "strconv"
)

func pow10u(e uint) uint {
    r := uint(1)
    for i := uint(0); i < e; i++ { r *= 10 }
    return r
}

func digitAt(n uint) uint {
    d := uint(1)
    block := 9 * pow10u(d-1) * d
    for n > block {
        n -= block
        d += 1
        block = 9 * pow10u(d-1) * d
    }
    idx := n - 1
    num := pow10u(d-1) + idx / d
    digitIndex := idx % d
    s := strconv.FormatUint(uint64(num), 10)
    return uint(s[digitIndex] - '0')
}

func main() {
    targets := []uint{1,10,100,1000,10000,100000,1000000}
    prod := uint64(1)
    for _, t := range targets {
        prod *= uint64(digitAt(t))
    }
    // fmt.Println(prod) // TODO: reveal locally
    fmt.Println("product d1*d10*...*d1000000 = <redacted>")

    if digitAt(12) != 1 { panic("sanity failed") }
}
