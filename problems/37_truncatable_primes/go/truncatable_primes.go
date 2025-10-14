// truncatable_primes.go
// Project Euler 37 - Truncatable Primes (Go)
package main

import (
    "fmt"
)

func isPrime(n int) bool {
    if n < 2 { return false }
    if n % 2 == 0 { return n == 2 }
    if n % 3 == 0 { return n == 3 }
    for f := 5; f * f <= n; f += 6 {
        if n % f == 0 || n % (f+2) == 0 { return false }
    }
    return true
}

func digitsCount(n int) int {
    if n == 0 { return 1 }
    k := 0
    for n > 0 { n /= 10; k++ }
    return k
}

func pow10i(e int) int {
    r := 1
    for e > 0 { r *= 10; e-- }
    return r
}

func growRight(layer []int) []int {
    var next []int
    for _, p := range layer {
        for _, d := range []int{1,3,7,9} {
            n := p*10 + d
            if isPrime(n) { next = append(next, n) }
        }
    }
    return next
}

func growLeft(layer []int) []int {
    var next []int
    for _, p := range layer {
        base := pow10i(digitsCount(p))
        for _, d := range []int{1,2,3,5,7,9} {
            n := d*base + p
            if isPrime(n) { next = append(next, n) }
        }
    }
    return next
}

func build(grow func([]int) []int) map[int]bool {
    res := make(map[int]bool)
    layer := []int{2,3,5,7}
    for {
        next := grow(layer)
        for _, n := range next { if n >= 10 { res[n] = true } }
        if len(next) == 0 { break }
        layer = next
    }
    return res
}

func main() {
    right := build(growRight)
    left  := build(growLeft)

    total := 0
    count := 0
    for n := range right {
        if left[n] && n >= 10 {
            total += n
            count++
        }
    }
    fmt.Printf("found %d truncatable primes (should be 11)\n", count)
    // fmt.Println(total) // TODO: reveal locally
    fmt.Println("sum of the eleven truncatable primes = <redacted>")
}
