// pandigital_multiples.go
// Project Euler 38 - Pandigital Multiples (Go)
package main

import "fmt"

func digits(n int) int {
    if n == 0 { return 1 }
    k := 0
    for n > 0 { n /= 10; k++ }
    return k
}

func pow10u(e int) int {
    r := 1
    for ; e > 0; e-- { r *= 10 }
    return r
}

func isPandigital(n int) bool {
    seen := 0
    count := 0
    for n > 0 {
        d := n % 10
        if d == 0 { return false }
        bit := 1 << uint(d)
        if seen & bit != 0 { return false }
        seen |= bit
        count++
        n /= 10
    }
    return count == 9 && seen == 0b1111111110
}

func concatProd(x int) (int, bool) {
    v, length, k := 0, 0, 1
    for length < 9 {
        p := x * k
        dp := digits(p)
        v = v * pow10u(dp) + p
        length += dp
        k++
    }
    if length == 9 && k-1 >= 2 {
        return v, true
    }
    return 0, false
}

func main() {
    best := 0
    for x := 1; x <= 9999; x++ {
        if v, ok := concatProd(x); ok && isPandigital(v) {
            if v > best { best = v }
        }
    }
    // fmt.Println(best) // TODO: reveal locally
    fmt.Println("largest pandigital concatenated product = <redacted>")
}
