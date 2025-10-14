// digit_fifth_powers.go
// Go reference solution for Project Euler 30. Final answer redacted.
package main

import "fmt"

func powu(b, e int) int {
    r := 1
    for i := 0; i < e; i++ { r *= b }
    return r
}

func solve(power int) int {
    t := [10]int{}
    for d := 0; d <= 9; d++ { t[d] = powu(d, power) }
    upper := 6 * t[9]
    total := 0
    for n := 2; n <= upper; n++ {
        s, m := 0, n
        for m > 0 {
            s += t[m%10]
            m /= 10
        }
        if s == n { total += n }
    }
    return total
}

func main() {
    if solve(4) != 19316 { panic("sanity failed") }
    ans := solve(5)
    // fmt.Println(ans) // TODO: reveal locally
    fmt.Println("Sum for power 5: [redacted]")
    _ = ans
}
