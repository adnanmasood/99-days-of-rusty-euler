// digit_cancelling_fractions.go
// Project Euler 33 - Digit Cancelling Fractions (Go)
package main

import "fmt"

func gcd(a, b int) int {
    for b != 0 {
        a, b = b, a%b
    }
    return a
}

func isCurious(n, d int) bool {
    nt, nu := n/10, n%10
    dt, du := d/10, d%10
    if nt == 0 || nu == 0 || dt == 0 || du == 0 {
        return false
    }
    check := func(kn, kd int) bool {
        return kd != 0 && n*kd == d*kn
    }
    if nu == dt && check(nt, du) { return true }
    if nt == du && check(nu, dt) { return true }
    if nt == dt && check(nu, du) { return true }
    if nu == du && check(nt, dt) { return true }
    return false
}

func main() {
    num, den := 1, 1
    for n := 10; n <= 99; n++ {
        for d := n + 1; d <= 99; d++ {
            if isCurious(n, d) {
                num *= n
                den *= d
            }
        }
    }
    g := gcd(num, den)
    ans := den / g
    // fmt.Println(ans) // TODO: reveal locally
    fmt.Println("denominator (lowest terms) = <redacted>")
}
